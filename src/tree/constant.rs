use serde::{Deserialize, Serialize};
use crate::tree::datum::DString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrVal {
    s: String,
    scanned_as_bytes: bool,
    res_string: DString,
}