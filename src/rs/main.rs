extern crate dotbc;
extern crate docopt;
extern crate rustc_serialize;

mod cmd;

use docopt::{Docopt};
use std::{env, process};

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

    if let Err(e) = cmd::run(env::args()) {
        println!("ERROR!!!! {:?}", e);
        process::exit(1);
    }
}
