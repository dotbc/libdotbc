extern crate dotbc;
extern crate docopt;
extern crate rustc_serialize;

macro_rules! printerrln {
    ($fmt:expr) => {{
        use ::std::io::Write;
        writeln!(&mut ::std::io::stderr(), $fmt).unwrap()
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        use ::std::io::Write;
        writeln!(&mut ::std::io::stderr(), $fmt, $($arg)*).unwrap()
    }};
}

mod cmd;

use std::{env, process};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn main() {
    match cmd::run(env::args()) {
        Ok(_) => {
            process::exit(0);
        }
        Err(cmd::Error::Docopt(docopt::Error::WithProgramUsage(_, usage))) => {
            printerrln!("Unknown command");
            printerrln!("{}", usage);
        }
        Err(cmd::Error::FileNotFound(ref f)) => {
            printerrln!("file not found `{}`", f);
        }
        Err(e) => {
            printerrln!("ERROR!!!! {:?}", e);
        }
    }

    process::exit(1);
}
