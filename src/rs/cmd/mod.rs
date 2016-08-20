
mod read_file;
mod write_file;

const USAGE: &'static str = "
dotbc

A utility providing access to the .bc file type.

Usage:
    dotbc <cmd> [<args>...]
    dotbc (-h | --help)

Options:
    -h, --help      Display this message

The most commonly used commands are:
    read-file        Read a file contained by a .bc archive
    write-file       Write a file to a .bc archive
";

use docopt::{self, Docopt};

#[derive(Debug)]
pub enum Error {
    Docopt(docopt::Error),
}

/// Run the command specified with the given set of arguments
pub fn run<I, S>(argv: I) -> Result<(), Error>
    where I: IntoIterator<Item=S>,
          S: AsRef<str>,
{
    // Copy the args... this is needed because we parse the args twice
    // depending on the specific command requested
    let argv: Vec<String> = argv.into_iter()
        .map(|arg| arg.as_ref().into())
        .collect();

    // Create a opt parser from the top level usage
    let docopt = try!(Docopt::new(USAGE))
        .help(false)
        .argv(&argv);

    // Extract the command and args
    let args = try!(docopt.parse());

    // Handle help requests
    if args.get_bool("-h") {
        println!("{}", USAGE);
        return Ok(());
    }

    // Dispatch to the subcommand
    match args.get_str("<cmd>") {
        "read-file" => read_file::run(argv),
        "write-file" => write_file::run(argv),
        _ => {
            // Unknown command
            unimplemented!();
        }
    }
}

/*
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
*/

impl From<docopt::Error> for Error {
    fn from(src: docopt::Error) -> Error {
        Error::Docopt(src)
    }
}
