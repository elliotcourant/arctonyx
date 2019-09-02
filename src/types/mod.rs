pub mod internal;
pub mod oid;

use internal::*;
use oid::*;
use std::borrow::Borrow;
use std::ptr::null;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct T {
    pub internal: internal::InternalType,
}

pub enum Types {
    Unknown
}

impl Types {
    pub fn t(&self) -> T {
        match self {
            Types::Unknown => {
                return T{
                    internal: InternalType {
                        family: Family::Unknown,
                        width: 0,
                        precision: 0,
                        array_dimensions: vec![],
                        visible_type: 0,
                        tuple_contents: vec![],
                        tuple_labels: vec![],
                        oid: Oid::Unknown,
                        array_contents: None
                    }
                }
            }
        }
    }
}