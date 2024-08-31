pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fs,
    io::{BufReader, Read},
    path,
};

#[derive(Clone, Debug, PartialEq)]
pub struct File<R> {
    pub name: String,
    pub reader: R,
}

impl<R: Read> File<R> {
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        let reader = &mut BufReader::new(&mut self.reader);
        reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

impl Opener for File<fs::File> {
    fn open<P: AsRef<path::Path>>(path: P) -> Result<Self, Error> {
        let name = path.as_ref().to_string_lossy().to_string();
        let reader = fs::File::open(path)?;

        Ok(Self { name, reader })
    }
}
