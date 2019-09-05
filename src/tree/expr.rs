use serde::{Deserialize, Serialize};
use crate::tree::constant::StrVal;
use crate::tree::datum::DString;

pub type Exprs = Vec<Expr>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    DString(DString),
    StrVal(StrVal)
}