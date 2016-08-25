
const USAGE: &'static str = "
dotbc write-file

Usage:
    dotbc write-file [-c] -f <dotbc> <path> [<data>]

Options:
    -c              Create the .bc file
    -f <dotbc>      Path to .bc file
    -h, --help      Show this message
";

use dotbc::DotBC;
use super::Error;
use docopt::Docopt;
use std::path::Path;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_f: String,
    flag_c: bool,
    arg_path: String,
    arg_data: Option<String>,
}

pub fn run(argv: Vec<String>) -> Result<(), Error> {
    let docopt = try!(Docopt::new(USAGE))
        .argv(argv);

    // Decode the args into a struct
    let args: Args = try!(docopt.decode());

    // Parse the path
    let path = Path::new(&args.flag_f);

    // The archive
    let mut dotbc;

    if !path.exists() {
        if !args.flag_c {
            return Err(Error::FileNotFound(args.flag_f.clone()));
        }

        dotbc = DotBC::new();
    } else {
        dotbc = try!(DotBC::open(path));
    }

    match args.arg_data {
        Some(ref data) => {
            dotbc.put(&args.arg_path[..], data.as_bytes());
        }
        None => unimplemented!(),
    }

    try!(dotbc.save(path));

    Ok(())
}
