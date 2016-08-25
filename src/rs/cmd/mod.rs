
mod read_file;
mod show_metadata;
mod write_file;

const USAGE: &'static str = "
dotbc

A utility providing access to the .bc file type.

Usage:
    dotbc <cmd> [<args>...]
    dotbc (-h | --help)
    dotbc --version

Options:
    -h, --help      Show this message
    --version       Print the version

The most commonly used commands are:
    read-file        Read a file contained by a .bc archive
    show-metadata    Show the .bc metadata as JSON
    write-file       Write a file to a .bc archive
";

use dotbc;
use docopt::{self, Docopt};

#[derive(Debug)]
pub enum Error {
    Docopt(docopt::Error),
    DotBC(dotbc::Error),
    FileNotFound(String),
}

/// Run the command specified with the given set of arguments
pub fn run<I, S>(argv: I) -> Result<(), Error>
    where I: IntoIterator<Item=S>,
          S: AsRef<str>,
{
    let cmd_args: Vec<String> = argv.into_iter()
        .map(|arg| arg.as_ref().into())
        .collect();

    let mut dispatch_args = vec![];

    for (i, arg) in cmd_args.iter().enumerate() {
        dispatch_args.push(arg.clone());

        if i > 0 && arg.chars().next() != Some('-') {
            // Found the cmd, done
            break;
        }
    }


    // Create a opt parser from the top level usage
    let docopt = try!(Docopt::new(USAGE))
        .help(false)
        .argv(dispatch_args);

    // Extract the command and args
    let args = match docopt.parse() {
        Ok(args) => args,
        Err(e) => return Err(e.into()),
    };

    // Handle help requests
    if args.get_bool("-h") {
        println!("{}", USAGE);
        return Ok(());
    }

    if args.get_bool("--version") {
        println!("dotbc {}", super::VERSION);
        return Ok(());
    }

    // Dispatch to the subcommand
    match args.get_str("<cmd>") {
        "read-file" => read_file::run(cmd_args),
        "write-file" => write_file::run(cmd_args),
        "show-metadata" => show_metadata::run(cmd_args),
        _ => {
            // Unknown command
            unimplemented!();
        }
    }
}

impl From<docopt::Error> for Error {
    fn from(src: docopt::Error) -> Error {
        Error::Docopt(src)
    }
}

impl From<dotbc::Error> for Error {
    fn from(src: dotbc::Error) -> Error {
        Error::DotBC(src)
    }
}
