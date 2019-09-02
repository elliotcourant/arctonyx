#[macro_use]
extern crate lazy_static;

use std::borrow::Borrow;

mod proto;
mod tree;
mod types;
mod parser;

fn main() {
    let startup: proto::startup_message::StartupMessage;
    let create: tree::create::CreateDatabase;
    let typ: types::T;
    let thing: parser::SqlParser;

    println!("Hello, world!");
}
