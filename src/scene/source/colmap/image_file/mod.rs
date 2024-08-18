pub mod image_files;

pub use crate::error::Error;
pub use image_files::*;

use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct ImageFile<R: io::Read> {
    pub file_name: String,
    pub file_reader: R,
}

impl<R: io::Read> ImageFile<R> {
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        use io::Read;

        let mut bytes = Vec::new();
        let reader = &mut io::BufReader::new(&mut self.file_reader);
        reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}
