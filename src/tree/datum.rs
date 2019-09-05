use serde::{Deserialize, Serialize};
use crate::tree::expr::Expr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DString {
    s: String,
}

impl DString {
    pub fn new(s: String) -> DString {
        return DString{
            s,
        };
    }

//    pub fn expr(e: Expr) -> Result<DString, bool> {
//        match e {
//
//        }
//    }
}
