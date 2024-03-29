use super::name::{Name, NameList};
use super::select::Direction;
use super::table_name::TableName;
use super::expr::Expr;
use crate::types::*;
use crate::tree::{Stmt, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatabase {
    pub if_not_exists: bool,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexElem {
    pub column: Name,
    pub direction: Direction,
}

pub type IndexElemList = Vec<IndexElem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndex {
    pub name: Name,
    pub table: TableName,
    pub unique: bool,
    pub inverted: bool,
    pub if_not_exists: bool,
    pub columns: IndexElemList,
    pub storing: NameList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Nullability {
    NotNull,
    Null,
    SilentNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nullable {
    pub nullability: Nullability,
    pub constraint_name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultExpr {
    pub expr: Expr,
    pub constraint_name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableDef {
    ColumnTableDef(ColumnTableDef),
    ConstraintTableDef(ConstraintTableDef),
}

pub type TableDefs = Vec<TableDef>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintTableDef {
    UniqueConstraintTableDef(UniqueConstraintTableDef),
    ForeignKeyConstraintTableDef(ForeignKeyConstraintTableDef),
    CheckConstraintTableDef(CheckConstraintTableDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnTableDef {
    pub name: Name,
    pub typ: T,
    pub is_serial: bool,
    pub nullable: Nullable,
    pub primary_key: bool,
    pub unique: bool,
    pub unique_constraint_name: Name,
    pub default_expr: Option<DefaultExpr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueConstraintTableDef {
    pub primary_key: bool,
    pub index: IndexTableDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyConstraintTableDef {
    pub name: Name,
    pub table: TableName,
    pub from_cols: NameList,
    pub to_cols: NameList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConstraintTableDef {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexTableDef {
    pub name: Name,
    pub columns: IndexElemList,
    pub storing: NameList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTable {
    pub if_not_exists: bool,
    pub table: TableName,
    pub defs: TableDefs,
}
