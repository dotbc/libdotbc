#![deny(warnings)]

extern crate zip;

mod c;

use zip::{ZipArchive, ZipWriter};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::collections::HashMap;

/// An interface to the DotBC file format
pub struct DotBC {
    files: HashMap<String, Vec<u8>>,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidArchive(&'static str),
    UnsupportedArchive(&'static str),
    FileNotFound,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl DotBC {
    pub fn new() -> DotBC {
        DotBC {
            files: HashMap::new(),
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<DotBC> {
        let file = try!(File::open(path));
        let mut zip = try!(ZipArchive::new(file));
        let mut dotbc = DotBC::new();

        for i in 0..zip.len() {
            let file = try!(zip.by_index(i));
            let name = file.name().into();
            let data = try!(file.bytes().collect());

            dotbc.files.insert(name, data);
        }

        Ok(dotbc)
    }

    pub fn get<P: AsRef<str>>(&self, path: P) -> Option<&[u8]> {
        self.files.get(path.as_ref()).map(|e| &**e)
    }

    pub fn put<K: Into<String>, V: Into<Vec<u8>>>(&mut self, path: K, val: V) {
        self.files.insert(path.into(), val.into());
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = try!(File::create(path));
        let mut zip = ZipWriter::new(file);

        for (path, data) in &self.files {
            try!(zip.start_file(&**path, zip::CompressionMethod::Bzip2));
            try!(zip.write(data));
        }

        Ok(())
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(src: zip::result::ZipError) -> Error {
        use zip::result::ZipError::*;

        match src {
            Io(e) => Error::Io(e),
            InvalidArchive(e) => Error::InvalidArchive(e),
            UnsupportedArchive(e) => Error::UnsupportedArchive(e),
            FileNotFound => Error::FileNotFound,
        }
    }
}

impl From<io::Error> for Error {
    fn from(src: io::Error) -> Error {
        Error::Io(src)
    }
}
