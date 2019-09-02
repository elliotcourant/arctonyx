use super::name::{Name, NameList};
use super::select::Direction;
use super::table_name::TableName;
use super::expr::Expr;
use crate::types::*;
use crate::tree::{Stmt, Statement};

pub struct CreateDatabase {
    pub if_not_exists: bool,
    pub name: Name,
}

pub struct IndexElem {
    pub column: Name,
    pub direction: Direction,
}

pub type IndexElemList = Vec<IndexElem>;

pub struct CreateIndex {
    pub name: Name,
    pub table: TableName,
    pub unique: bool,
    pub inverted: bool,
    pub if_not_exists: bool,
    pub columns: IndexElemList,
    pub storing: NameList,
}

#[derive(Debug, Clone)]
pub enum Nullability {
    NotNull,
    Null,
    SilentNull,
}

#[derive(Debug, Clone)]
pub struct Nullable {
    pub nullability: Nullability,
    pub constraint_name: Name,
}

#[derive(Debug, Clone)]
pub struct DefaultExpr {
    pub expr: Expr,
    pub constraint_name: Name,
}

#[derive(Debug, Clone)]
pub enum TableDef {
    ColumnTableDef(ColumnTableDef)
}

pub type TableDefs = Vec<TableDef>;

#[derive(Debug, Clone)]
pub struct ColumnTableDef {
    pub name: Name,
    pub typ: T,
    pub is_serial: bool,
    pub nullable: Nullable,
    pub primary_key: bool,
    pub unique: bool,
    pub unique_constraint_name: Name,
    pub default_expr: DefaultExpr,
}

#[derive(Debug, Clone)]
pub struct CreateTable {
    pub if_not_exists: bool,
    pub table: TableName,
    pub defs: TableDefs,
}

impl Statement for CreateTable {}