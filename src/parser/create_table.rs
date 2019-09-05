use super::*;
use crate::tree::select::Direction;

macro_rules! parser_err {
    ($MSG:expr) => {
        Err(ParserError::ParserError($MSG.to_string()))
    };
}

impl SqlParser {
    pub(crate) fn parse_create_table(&mut self) -> Result<Node, ParserError> {
        let if_not_exists = self.parse_keywords(vec!["IF", "NOT", "EXISTS"]);
        let table = match self.parse_table_name() {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        let columns = self.parse_create_table_columns(table.clone());
        Ok(Node::CreateTable {
            0: CreateTable {
                if_not_exists,
                table,
                defs: columns.unwrap(),
            }
        })
    }

    fn parse_create_table_columns(&mut self, table_name: TableName) -> Result<TableDefs, ParserError> {
        let mut columns = vec![];
        if !self.parser.consume_token(&Token::LParen) || self.parser.consume_token(&Token::RParen) {
            return Ok(columns);
        }

        loop {
            if let Some(constraint) = self.parse_optional_table_constraint(table_name.clone())? {
                columns.push(TableDef::ConstraintTableDef(constraint))
            } else if let Some(Token::Word(column_name)) = self.parser.peek_token() {
                let mut column = ColumnTableDef {
                    name: column_name.clone().to_string(),
                    typ: Types::Unknown.t(),
                    is_serial: false,
                    nullable: Nullable { nullability: Nullability::NotNull, constraint_name: "".to_string() },
                    primary_key: false,
                    unique: false,
                    unique_constraint_name: "".to_string(),
                    default_expr: None,
                };

                self.parser.next_token();
                let data_type = self.parser.next_token();

                // What should a column default to if its not specified?
                column.nullable = Nullable {
                    nullability: Nullability::SilentNull,
                    constraint_name: "".to_string(),
                };

                loop {
                    match self.parser.peek_token() {
                        None | Some(Token::Comma) | Some(Token::RParen) => break,
                        _ => {
                            if self.parse_keywords(vec!["NOT", "NULL"]) {
                                column.nullable = Nullable {
                                    nullability: Nullability::NotNull,
                                    constraint_name: format!("cn_{}_not_null_{}", table_name.table_name, column.name),
                                }
                            } else if self.parser.parse_keyword("NULL") {
                                column.nullable = Nullable {
                                    nullability: Nullability::Null,
                                    constraint_name: format!("cn_{}_null_{}", table_name.table_name, column.name),
                                }
                            } else if self.parse_keywords(vec!["PRIMARY", "KEY"]) {
                                column.primary_key = true;
                                column.unique = true;
                                let name = format!("pk_{}_{}", table_name.table_name, column.name);
                                column.unique_constraint_name = name.clone();
                                column.nullable = Nullable {
                                    nullability: Nullability::NotNull,
                                    constraint_name: format!("cn_pk_{}_not_null_{}", table_name.table_name, column.name),
                                };
                                columns.push(TableDef::ConstraintTableDef(
                                    ConstraintTableDef::UniqueConstraintTableDef(
                                        UniqueConstraintTableDef {
                                            primary_key: true,
                                            index: IndexTableDef {
                                                name: name.clone(),
                                                columns: vec![IndexElem {
                                                    column: column_name.clone().to_string(),
                                                    direction: Direction::DefaultDirection,
                                                }],
                                                storing: vec![],
                                            },
                                        }
                                    )
                                ))
                            } else if self.parser.parse_keyword("UNIQUE") {
                                column.unique = true;
                                let name = format!("uq_{}_{}", table_name.table_name, column.name);
                                column.unique_constraint_name = name.clone();
                                columns.push(TableDef::ConstraintTableDef(
                                    ConstraintTableDef::UniqueConstraintTableDef(
                                        UniqueConstraintTableDef {
                                            primary_key: false,
                                            index: IndexTableDef {
                                                name: name.clone(),
                                                columns: vec![IndexElem {
                                                    column: column_name.clone().to_string(),
                                                    direction: Direction::DefaultDirection,
                                                }],
                                                storing: vec![],
                                            },
                                        }
                                    )
                                ))
                            } else if self.parser.parse_keyword("REFERENCES") {
                                let foreign_table = match self.parse_table_name() {
                                    Ok(t) => t,
                                    Err(e) => return Err(e)
                                };
                                match self.parse_parenthesized_column_list() {
                                    Ok(to_columns) => {
                                        columns.push(TableDef::ConstraintTableDef(
                                            ConstraintTableDef::ForeignKeyConstraintTableDef(
                                                ForeignKeyConstraintTableDef {
                                                    name: format!("fk_{}_{}_{}", table_name.table_name, column.name, foreign_table.table_name),
                                                    table: foreign_table,
                                                    from_cols: vec![column_name.clone().to_string()],
                                                    to_cols: to_columns,
                                                }
                                            )
                                        ))
                                    }
                                    Err(e) => return Err(e)
                                }
                            } else if self.parser.parse_keyword("CHECK") {
                                return parser_err!("not implemented");
                            } else {
                                return self.expected("column option", self.parser.peek_token());
                            }
                        }
                    }
                }

                columns.push(TableDef::ColumnTableDef(column))
            } else {
                return self.expected("column name or constraint definition", self.parser.peek_token());
            }

            let comma = self.parser.consume_token(&Token::Comma);
            if self.parser.consume_token(&Token::RParen) {
                // allow a trailing comma, even though it's not in standard
                break;
            } else if !comma {
                return self.expected("',' or ')' after column definition", self.parser.peek_token());
            }
        }

        Ok(columns)
    }

    pub fn parse_optional_table_constraint(&mut self, table_name: TableName) -> Result<Option<ConstraintTableDef>, ParserError> {
        let name = if self.parser.parse_keyword("CONSTRAINT") {
            Some(self.parser.parse_identifier())
        } else {
            None
        };

        match self.parser.next_token() {
            Some(Token::Word(ref k)) if k.keyword == "PRIMARY".to_string() || k.keyword == "UNIQUE".to_string() => {
                let is_primary = k.keyword == "PRIMARY".to_string();
                if is_primary {
                    self.parser.expect_keyword("KEY")?;
                }
                let columns = self.parser.parse_parenthesized_column_list(Mandatory);
                Ok(Some(ConstraintTableDef::UniqueConstraintTableDef {
                    0: UniqueConstraintTableDef {
                        primary_key: is_primary,
                        index: IndexTableDef {
                            name: "".to_string(),
                            columns: vec![],
                            storing: vec![],
                        },
                    }
                }))
            }
            unexpected => {
                if name.is_some() {
                    self.expected("PRIMARY, UNIQUE, FOREIGN, or CHECK", unexpected)
                } else {
                    self.parser.prev_token();
                    Ok(None)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn does_parse(sql: &str) {
        let result = SqlParser::parse_sql(String::from(sql));
        assert!(!result.is_err());
        let j = serde_json::to_string(&result.unwrap());
        assert!(!j.is_err());
        println!("{}", sql.to_string());
        println!("{}", j.unwrap());
    }

    #[test]
    fn test_create_table() {
        does_parse("create table \"public\".\"test\" (id int primary key);")
    }

    #[test]
    fn test_create_table_no_pk() {
        does_parse("create table test (id int);");
    }

    #[test]
    fn test_create_table_unique() {
        does_parse( "create table test (id int primary key, name text unique);");
    }

    #[test]
    fn test_create_table_fk() {
        does_parse( "create table users (user_id int primary key, account_id int references accounts (account_id));");
    }
}