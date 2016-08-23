
const USAGE: &'static str = "
dotbc show-metadata

Usage:
    dotbc show-metadata [-f <dotbc>]

Options:
    -f <dotbc-file>     Path to .bc file (default: STDIN)
    -h, --help          Show this message
";

use super::Error;
use dotbc::DotBC;
use docopt::Docopt;
use std::path::Path;
use std::str;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_f: Option<String>,
}

pub fn run(argv: Vec<String>) -> Result<(), Error> {
    println!("SHOW MEtaDATA");

    let docopt = try!(Docopt::new(USAGE))
        .argv(argv);

    // Decode the args into a struct
    let args: Args = try!(docopt.decode());

    let path = match args.flag_f {
        Some(ref f) => Path::new(f),
        None => unimplemented!(), // STDIN
    };

    let dotbc = try!(DotBC::open(path));

    match dotbc.get("metadata.json").map(str::from_utf8) {
        Some(Ok(val)) => println!("{}", val),
        None => println!("{{}}"),
        _ => unimplemented!(),
    }

    Ok(())
}
