use super::*;
use serde::{Deserialize, Serialize};
use crate::tree::with::With;
use crate::tree::select::TableExpr;
use crate::tree::name::NameList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insert {
    with: Option<With>,
    table: TableExpr,
    columns: NameList,
}