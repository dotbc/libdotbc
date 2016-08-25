
const USAGE: &'static str = "
dotbc read-file

Usage:
    dotbc read-file [-f <dotbc>] <path>

Options:
    -f <dotbc-file>     Path to .bc file (default: STDIN)
    -h, --help          Show this message
";

use dotbc::DotBC;
use super::Error;
use docopt::Docopt;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_f: Option<String>,
    arg_path: String,
}

pub fn run(argv: Vec<String>) -> Result<(), Error> {
    let docopt = try!(Docopt::new(USAGE))
        .argv(argv);

    // Decode the args into a struct
    let args: Args = try!(docopt.decode());

    let path = match args.flag_f {
        Some(ref f) => Path::new(f),
        None => unimplemented!(), // STDIN
    };

    let dotbc = try!(DotBC::open(path));

    match dotbc.get(args.arg_path) {
        Some(val) => {
            try!(io::stdout().write_all(val));
            try!(io::stdout().write(b"\n"));
        }
        None => println!("{{}}"),
    }

    Ok(())
}
