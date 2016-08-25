#![deny(warnings)]

extern crate zip;

mod c;

use zip::{ZipArchive, ZipWriter};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::collections::{hash_map, HashMap};

/// An interface to the .bc file format
pub struct DotBC {
    files: HashMap<String, Vec<u8>>,
}

/// Iterator over the files of a .bc archive.
pub struct Files<'a> {
    iter: hash_map::Keys<'a, String, Vec<u8>>,
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

    /// Open the .bc archive located at the given path
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

    /// Return an iterator over the files contained by the .bc archive.
    pub fn files(&self) -> Files {
        Files { iter: self.files.keys() }
    }

    /// Return the contents of the file contained by the .bc archive at the
    /// given path.
    pub fn get<P: AsRef<str>>(&self, path: P) -> Option<&[u8]> {
        self.files.get(path.as_ref()).map(|e| &**e)
    }

    /// Put the given data into the .bc archive at the given path.
    pub fn put<K: Into<String>, V: Into<Vec<u8>>>(&mut self, path: K, val: V) {
        self.files.insert(path.into(), val.into());
    }

    /// Save the .bc archive to disk at the given path.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = try!(File::create(path));
        let mut zip = ZipWriter::new(file);

        for (path, data) in &self.files {
            try!(zip.start_file(&**path, zip::CompressionMethod::Deflated));
            try!(zip.write(data));
        }

        Ok(())
    }
}

impl<'a> Iterator for Files<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.iter.next().map(AsRef::as_ref)
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
