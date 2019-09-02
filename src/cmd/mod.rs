pub enum Argument {
    Directory,
    PgListenAddress,
    RaftListenAddress,
    Peers,
}

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new(args: Vec<String>) -> Args {
        println!("{:?}", args);
        return Args{
            args,
        }
    }
}