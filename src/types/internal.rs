use super::T;
use super::oid::Oid;

pub enum Family {
    Bool,
    Int,
    Float,
    Decimal,
    Date,
    Timestamp,
    Interval,
    String,
    Bytes,
    TimestampTZ,
    Oid,
    Unknown,
    Uuid,
    Array,
    INet,
    Time,
    Json,
    Tuple,
    Bit,
    Any,
}

pub struct InternalType {
    pub family: Family,
    pub width: i32,
    pub precision: i32,
    pub array_dimensions: Vec<i32>,
    pub visible_type: i32,
    pub tuple_contents: Vec<T>,
    pub tuple_labels: Vec<String>,
    pub oid: Oid,
    pub array_contents: Option<Box<T>>,
}