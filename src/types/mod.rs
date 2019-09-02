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