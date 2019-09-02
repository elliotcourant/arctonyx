use crate::tree::create::{CreateDatabase, CreateTable};

pub mod create;
pub mod name;
pub mod select;
pub mod table_name;
pub mod expr;

pub enum Stmt {
    CreateTable(CreateTable)
}

pub trait Statement {

}