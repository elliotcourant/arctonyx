use rocksdb::{DB, Options};
use std::env;

mod proto;
mod tree;
mod types;
mod parser;
mod cmd;

fn main() {
    let args = cmd::Args::new(env::args().collect());

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
