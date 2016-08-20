
const USAGE: &'static str = "
dotbc read-file

Usage:
    dotbc read-file <path> (<bc-file> | -)
";

use super::Error;
use docopt::Docopt;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_path: String,
}

pub fn run(argv: Vec<String>) -> Result<(), Error> {
    let docopt = try!(Docopt::new(USAGE))
        .argv(argv);

    // Decode the args into a struct
    let args: Args = try!(docopt.decode());

    println!("read-file: {:?}", args);
    Ok(())
}
