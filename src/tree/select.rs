use serde::{Deserialize, Serialize};
use crate::tree::name::*;
use crate::tree::table_name::TableName;
use crate::tree::with::With;
use crate::tree::values::ValuesClause;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Select {
    with: Option<With>,
    select: SelectStatement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectStatement {
    ValuesClause(ValuesClause)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    DefaultDirection,
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasClause {
    alias: Name,
    cols: NameList,
}

// TableExpr represents a table expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableExpr {
    TableName(TableName)
}