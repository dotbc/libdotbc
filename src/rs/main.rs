extern crate dotbc;
extern crate docopt;
extern crate rustc_serialize;

mod cmd;

use std::{env, process};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn main() {
    match cmd::run(env::args()) {
        Err(cmd::Error::Docopt(docopt::Error::WithProgramUsage(_, usage))) => {
            println!("Unknown command");
            println!("{}", usage);
            process::exit(1);
        }
        Err(e) => {
            println!("ERROR!!!! {:?}", e);
            process::exit(1);
        }
        _ => {}
    }
}
