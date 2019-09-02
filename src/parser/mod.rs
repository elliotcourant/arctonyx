// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! SQL Parser
//!
//! Note that most SQL parsing is now delegated to the sqlparser crate, which handles ANSI SQL but
//! this module contains DataFusion-specific SQL extensions.

use sqlparser::dialect::*;
use sqlparser::sqlast::*;
use sqlparser::sqlparser::*;
use sqlparser::sqltokenizer::*;
use crate::tree::create::CreateTable;

macro_rules! parser_err {
    ($MSG:expr) => {
        Err(ParserError::ParserError($MSG.to_string()))
    };
}

#[derive(Debug, Clone)]
pub enum FileType {
    NdJson,
    Parquet,
    CSV,
}

#[derive(Debug, Clone)]
pub enum Node {
    /// ANSI SQL AST node
    ANSI(ASTNode),
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
        let dialect = GenericSqlDialect {};
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
            return parser_err!("invalid query")
        }
    }

    fn parse_create(&mut self) -> Result<Node, ParserError> {
        if self.parser.parse_keyword("TABLE") {
            return parser_err!("could not create table")
        } else if self.parser.parse_keyword("DATABASE") {
            return parser_err!("could not create database")
        } else {
            return parser_err!("could not create object")
        }
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
        let result = SqlParser::parse_sql(String::from("create table"));
        assert!(!result.is_err());
        println!("test");
    }
}