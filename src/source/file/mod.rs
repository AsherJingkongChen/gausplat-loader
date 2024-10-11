pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fs,
    io::{BufReader, BufWriter, Read, Write},
    path,
};

#[derive(Clone, Debug, PartialEq)]
pub struct File<S> {
    pub name: String,
    pub stream: S,
}

impl<R: Read> File<R> {
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        let reader = &mut BufReader::new(&mut self.stream);
        reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

impl<W: Write> File<W> {
    pub fn write(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), Error> {
        let writer = &mut BufWriter::new(&mut self.stream);
        writer.write_all(bytes)?;

        Ok(())
    }
}

impl Opener for File<fs::File> {
    fn open(path: impl AsRef<path::Path>) -> Result<Self, Error> {
        let name = path
            .as_ref()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let stream = fs::OpenOptions::new()
            .read(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        Ok(Self { name, stream })
    }
}

impl<R: Default> Default for File<R> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            stream: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read() {
        use super::*;

        let input = b"Hello, World!";
        let mut file = File {
            name: Default::default(),
            stream: std::io::Cursor::new(input),
        };

        let output = file.read().unwrap();
        assert_eq!(output, input);
    }

    #[test]
    fn write() {
        use super::*;

        let input = b"Hello, World!";
        let mut file = File {
            name: Default::default(),
            stream: std::io::Cursor::new(Vec::new()),
        };

        file.write(input).unwrap();
        let output = file.read().unwrap();
        assert_eq!(output, input);
    }
}
