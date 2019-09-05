#[macro_use]
use sqlparser::dialect::*;
use sqlparser::parser::*;
use sqlparser::tokenizer::*;
use crate::tree::create::*;
use crate::tree::table_name::{TableName, TableNamePrefix};
use crate::types::{T, Types};
use crate::types::internal::{InternalType, Family};
use crate::types::oid::Oid;
use std::borrow::Borrow;
use crate::tree::name::{Name, NameList};
use sqlparser::parser::IsOptional::{Mandatory, Optional};
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
use std::time::Instant;

mod create_table;

macro_rules! parser_err {
    ($MSG:expr) => {
        Err(ParserError::ParserError($MSG.to_string()))
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let start = Instant::now();
        let mut parser = SqlParser::new(sql)?;
        let result = parser.parse();
        println!("parse_time: {:?}", start.elapsed());
        return result;
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

    pub fn parse_table_name(&mut self) -> Result<TableName, ParserError> {
        let mut table_name = TableName {
            table_name: "".to_string(),
            prefix: TableNamePrefix {
                catalog_name: "".to_string(),
                schema_name: "".to_string(),
                explicit_catalog: false,
                explicit_schema: false,
            },
        };
        let mut items = vec![];
        loop {
            match self.parser.next_token() {
                Some(Token::Word(w)) => items.push(w.value),
                unexpected => return self.expected("identifier", unexpected)
            }
            if !self.parser.consume_token(&Token::Period) {
                break;
            }
        }
        match items.len() {
            1 => table_name.table_name = items[0].clone(),
            2 => {
                table_name.table_name = items[1].clone();
                table_name.prefix.schema_name = items[0].clone();
                table_name.prefix.explicit_schema = true;
            }
            3 => {
                table_name.table_name = items[2].clone();
                table_name.prefix.schema_name = items[1].clone();
                table_name.prefix.explicit_schema = true;
                table_name.prefix.catalog_name = items[0].clone();
                table_name.prefix.explicit_catalog = true;
            }
            _ => return parser_err!("table name is not valid")
        }
        Ok(table_name)
    }

    pub fn parse_parenthesized_column_list_sorted(&mut self) -> Result<IndexElemList, ParserError> {
        return parser_err!("not implemented");
    }

    pub fn parse_parenthesized_column_list(&mut self) -> Result<NameList, ParserError> {
        match self.parser.parse_parenthesized_column_list(IsOptional::Mandatory) {
            Ok(columns) => Ok(columns),
            Err(err) => Err(err)
        }
    }

    /// Report unexpected token
    fn expected<T>(&self, expected: &str, found: Option<Token>) -> Result<T, ParserError> {
        parser_err!(format!(
            "Expected {}, found: {}",
            expected,
            found.map_or_else(|| "EOF".to_string(), |t| format!("{}", t))
        ))
    }

    pub fn parse_keywords(&mut self, keywords: Vec<&'static str>) -> bool {
        return self.parser.parse_keywords(keywords);
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
        let j = serde_json::to_string(&stmt);
        assert!(!j.is_err());
        println!("{}", sql.to_string());
        println!("{}", j.unwrap());
    }
}