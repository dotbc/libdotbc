
const USAGE: &'static str = "
dotbc write-file

Usage:
    dotbc write-file [-c] -f <dotbc> <path> [<data>]

Options:
    -c              Create the .bc file
    -f <dotbc>      Path to .bc file
    -h, --help      Show this message
";

use super::Error;
use docopt::Docopt;

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

    println!("write-file: {:?}", args);
    Ok(())
}

