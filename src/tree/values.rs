use serde::{Deserialize, Serialize};
use crate::tree::expr::Exprs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesClause {
    rows: Vec<Exprs>
}