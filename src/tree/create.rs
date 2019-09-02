use super::name::{Name, NameList};
use super::select::Direction;
use super::table_name::TableName;
use super::expr::Expr;
use crate::types::*;

pub struct CreateDatabase {
    if_not_exists: bool,
    name: Name,
}

pub struct IndexElem {
    column: Name,
    direction: Direction,
}

pub type IndexElemList = Vec<IndexElem>;

pub struct CreateIndex {
    name: Name,
    table: TableName,
    unique: bool,
    inverted: bool,
    if_not_exists: bool,
    columns: IndexElemList,
    storing: NameList,
}

pub enum Nullability {
    NotNull,
    Null,
    SilentNull,
}

pub struct Nullable {
    nullability: Nullability,
    constraint_name: Name,
}

pub struct DefaultExpr {
    expr: Box<dyn Expr>,
    constraint_name: Name,
}

pub struct ColumnTableDef {
    name: Name,
    typ: T,
    is_serial: bool,
    nullable: Nullable,
    primary_key: bool,
    unique: bool,
    unique_constraint_name: Name,
    default_expr: DefaultExpr,
}