#[macro_use]
extern crate lazy_static;

use std::borrow::Borrow;

mod proto;
mod tree;
mod types;

fn main() {
    let startup: proto::startup_message::StartupMessage;
    let create: tree::create::CreateDatabase;
    let typ: types::T;

    println!("Hello, world!");
}
