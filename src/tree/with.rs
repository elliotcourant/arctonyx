use super::*;
use serde::{Deserialize, Serialize};
use crate::tree::select::AliasClause;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct With {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTE {
    name: AliasClause,
    stmt: Stmt,
}