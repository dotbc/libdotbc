
const USAGE: &'static str = "
dotbc ls-files

Usage:
    dotbc ls-files -f <dotbc>

Options:
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
}

pub fn run(argv: Vec<String>) -> Result<(), Error> {
    let docopt = try!(Docopt::new(USAGE))
        .argv(argv);

    // Decode the args into a struct
    let args: Args = try!(docopt.decode());

    // Parse the path
    let path = Path::new(&args.flag_f);

    if !path.exists() {
        return Err(Error::FileNotFound(args.flag_f.clone()));
    }

    // Open the archive
    let dotbc = try!(DotBC::open(path));

    for file in dotbc.files() {
        println!("{}", file);
    }

    Ok(())
}
