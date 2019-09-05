use rocksdb::{DB, Options};
use std::env;

mod proto;
mod tree;
mod types;
mod parser;
mod cmd;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::time::Instant;

fn main() {
    let args = cmd::Args::new(env::args().collect());
    let stmt = parser::SqlParser::parse_sql("CREATE TABLE accounts (account_id BIGINT NOT NULL PRIMARY KEY, name TEXT);".to_string());
    if stmt.is_err() {
        panic!(stmt.err())
    }
    let s = stmt.unwrap();
    let j = serde_json::to_string(&s);
    if j.is_err() {
        panic!(j.err())
    }
    println!("{}", j.unwrap());

    let startup: proto::startup_message::StartupMessage;
    let create: tree::create::CreateDatabase;
    let typ: types::T;
    let thing: parser::SqlParser;

// NB: db is automatically closed at end of lifetime
    let path = "tmp";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => {
                println!("retrieved value {}", value.to_utf8().unwrap())
            },
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);

    println!("Hello, world!");
}
