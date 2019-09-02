use sqlparser::dialect::*;
use sqlparser::parser::*;
use sqlparser::tokenizer::*;
use crate::tree::create::{CreateTable, TableDefs, TableDef, ColumnTableDef, Nullable, DefaultExpr, Nullability, ForeignKeyConstraintTableDef, UniqueConstraintTableDef, ConstraintTableDef, IndexTableDef, IndexElem, IndexElemList};
use crate::tree::table_name::TableName;
use crate::types::{T, Types};
use crate::types::internal::{InternalType, Family};
use crate::types::oid::Oid;
use std::borrow::Borrow;
use crate::tree::name::Name;
use sqlparser::parser::IsOptional::Mandatory;

macro_rules! parser_err {
    ($MSG:expr) => {
        Err(ParserError::ParserError($MSG.to_string()))
    };
}

#[derive(Debug, Clone)]
pub enum Node {
    /// DDL for creating an external table in DataFusion
    CreateTable(CreateTable),
}

/// SQL Parser
pub struct SqlParser {
    parser: Parser,
}

impl SqlParser {
    /// Parse the specified tokens
    pub fn new(sql: String) -> Result<Self, ParserError> {
        let dialect = PostgreSqlDialect {};
        let mut tokenizer = Tokenizer::new(&dialect, &sql);
        let tokens = tokenizer.tokenize()?;
        Ok(SqlParser {
            parser: Parser::new(tokens),
        })
    }

    /// Parse a SQL statement and produce an Abstract Syntax Tree (AST)
    pub fn parse_sql(sql: String) -> Result<Node, ParserError> {
        let mut parser = SqlParser::new(sql)?;
        parser.parse()
    }

    /// Parse a new expression
    pub fn parse(&mut self) -> Result<Node, ParserError> {
        self.parse_expr(0)
    }

    /// Parse tokens until the precedence changes
    fn parse_expr(&mut self, precedence: u8) -> Result<Node, ParserError> {
        let mut expr = self.parse_prefix()?;
        loop {
            let next_precedence = self.parser.get_next_precedence()?;
            if precedence >= next_precedence {
                break;
            }

            if let Some(infix_expr) = self.parse_infix(expr.clone(), next_precedence)? {
                expr = infix_expr;
            }
        }
        Ok(expr)
    }

    /// Parse an expression prefix
    fn parse_prefix(&mut self) -> Result<Node, ParserError> {
        if self.parser.parse_keyword("CREATE") {
            self.parse_create()
        } else {
            return parser_err!("invalid query");
        }
    }

    fn parse_create(&mut self) -> Result<Node, ParserError> {
        if self.parser.parse_keyword("TABLE") {
            self.parse_create_table()
        } else if self.parser.parse_keyword("DATABASE") {
            return parser_err!("could not create database");
        } else {
            return parser_err!("could not create object");
        }
    }

    fn parse_create_table(&mut self) -> Result<Node, ParserError> {
        let if_not_exists = self.parser.parse_keywords(vec!["IF", "NOT", "EXISTS"]);
        let table_name = self.parser.parse_object_name().unwrap();
        let table = TableName::new(table_name.to_string());
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
                self.parser.next_token();
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

        columns.push(TableDef::ColumnTableDef {
            0: ColumnTableDef {
                name: "".to_string(),
                typ: Types::Unknown.t(),
                is_serial: false,
                nullable: Nullable {
                    nullability: Nullability::NotNull,
                    constraint_name: "".to_string(),
                },
                primary_key: false,
                unique: false,
                unique_constraint_name: "".to_string(),
                default_expr: None,
            }
        });

        columns.push(TableDef::ConstraintTableDef {
            0: ConstraintTableDef::ForeignKeyConstraintTableDef {
                0: ForeignKeyConstraintTableDef {
                    name: "".to_string(),
                    table: table_name,
                    from_cols: vec![],
                    to_cols: vec![],
                },
            }
        });

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
                Ok(Some(ConstraintTableDef::UniqueConstraintTableDef{
                    0: UniqueConstraintTableDef {
                        primary_key: is_primary,
                        index: IndexTableDef {
                            name: "".to_string(),
                            columns: vec![],
                            storing: vec![]
                        }
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

    pub fn parse_parenthesized_column_list(&mut self) -> Result<IndexElemList, ParserError> {
        return parser_err!("not implemented")
    }


    /// Report unexpected token
    fn expected<T>(&self, expected: &str, found: Option<Token>) -> Result<T, ParserError> {
        parser_err!(format!(
            "Expected {}, found: {}",
            expected,
            found.map_or_else(|| "EOF".to_string(), |t| format!("{}", t))
        ))
    }

    pub fn parse_infix(
        &mut self,
        _expr: Node,
        _precedence: u8,
    ) -> Result<Option<Node>, ParserError> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        let sql = "create table \"public\".\"test\" (id int primary key);";
        let result = SqlParser::parse_sql(String::from(sql));
        assert!(!result.is_err());
        let stmt = result.unwrap();
        match stmt {
            Node::CreateTable(v) => {
                for item in v.defs {
                    match item {
                        TableDef::ColumnTableDef(c) => {
                            println!("found column {}", c.name)
                        }
                        TableDef::ConstraintTableDef(c) => {
                            match c {
                                ConstraintTableDef::ForeignKeyConstraintTableDef(x) => {
                                    println!("found foreign key constraint")
                                }
                                ConstraintTableDef::UniqueConstraintTableDef(x) => {
                                    println!("found unique constraint")
                                }
                                ConstraintTableDef::CheckConstraintTableDef(x) => {
                                    println!("found check constraint")
                                }
                            }
                        }
                    }
                }
                println!("create table")
            }
            _ => {}
        }
        println!("{}", sql.to_string());
    }
}