
const USAGE: &'static str = "
dotbc write-file

Usage:
    dotbc write-file <path> (<bc-file> | -)
";

use super::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    println!("write-file");
    Ok(())
}

