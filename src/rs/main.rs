extern crate dotbc;
extern crate docopt;

use docopt::Docopt;
use std::process;

const USAGE: &'static str = "
dotbc

Usage:
    dotbc <cmd> [<args>...]
    dotbc (-h | --help)

Options:
    -h, --help      Display this message

The most commonly used commands are:
    read-file        Read a file contained by a .bc archive
    write-file       Write a file to a .bc archive
";

const READ_FILE: &'static str = "
dotbc read-file

Usage:
    dotbc read-file <path> (<bc-file> | -)
";

pub fn main() {
    let res = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    if res.get_bool("-h") {
        println!("{}", USAGE);
        process::exit(0);
    }

    println!("{:?}", res);
    println!("CMD: {:?}", res.get_str("<cmd>"));

    if res.get_str("<cmd>") == "read-file" {
        let mut args: Vec<String> = vec!["dotbc".into(), "read-file".into()];
        args.append(&mut res.get_vec("<args>").iter().map(|&s| s.into()).collect());
        println!("ARGS: {:?}", args);

        let res = Docopt::new(READ_FILE)
            .and_then(|d| d.argv(args).parse())
            ;

        println!("ZOMG: {:?}", res);
    } else {
        println!("NOPE");
    }
}
