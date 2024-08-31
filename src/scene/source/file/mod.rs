pub mod files;

pub use crate::function::Opener;
pub use crate::error::Error;
pub use files::*;

use std::{fs, io, path};

#[derive(Clone, Debug, PartialEq)]
pub struct File<R: io::Read> {
    pub name: String,
    pub reader: R,
}

impl<R: io::Read> File<R> {
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        use io::Read;

        let mut bytes = Vec::new();
        let reader = &mut io::BufReader::new(&mut self.reader);
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
