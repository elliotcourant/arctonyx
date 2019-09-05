use crate::tree::create::{CreateDatabase, CreateTable};
use serde::{Deserialize, Serialize};

pub mod create;
pub mod name;
pub mod select;
pub mod table_name;
pub mod expr;
pub mod insert;
pub mod with;
pub mod values;
pub mod constant;
pub mod datum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    CreateTable(CreateTable)
}

pub trait Statement {

}