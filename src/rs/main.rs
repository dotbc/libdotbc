extern crate dotbc;
extern crate docopt;

use docopt::Docopt;
use std::process;

const TOP: &'static str = "
Usage:
    dotbc --help
    dotbc <cmd> [<args>...]

";
/*
Options:
    -h, --help      Display this message
The most commonly used commands are:
    read        Read a file contained by a .bc archive
    write       Write a file to a .bc archive
";
*/

pub fn main() {
    let res = Docopt::new(TOP)
        .and_then(|d| d.parse())
        .unwrap_or_else(|_| {
            println!("Invalid arguments\n");
            println!("{}", TOP);
            process::exit(1);
        });

    if res.get_bool("-h") {
        println!("{}", TOP);
        process::exit(0);
    }

    println!("GOT: {:?}", res);
}
