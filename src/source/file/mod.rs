pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fs,
    io::{BufReader, BufWriter, Read, Seek, Write},
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

impl<S: Seek> File<S> {
    pub fn rewind(&mut self) -> Result<(), Error> {
        Ok(self.stream.rewind()?)
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
            .create(true)
            .read(true)
            .truncate(false)
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
    fn open() {
        use super::*;

        let source = "examples/data/hello-world/ascii.txt";
        let mut file = File::open(source).unwrap();

        let target = b"Hello, World!";
        let output = file.read().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read() {
        use super::*;

        let source =
            include_bytes!("../../../examples/data/hello-world/ascii.txt");
        let mut file = File {
            name: Default::default(),
            stream: std::io::Cursor::new(source),
        };

        let target = source;
        let output = file.read().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn write_and_rewind() {
        use super::*;

        let source =
            include_bytes!("../../../examples/data/hello-world/ascii.txt");
        let mut file = File::<std::io::Cursor<Vec<u8>>>::default();

        let target = source;
        file.write(source).unwrap();
        file.rewind().unwrap();
        let output = file.stream.into_inner();
        assert_eq!(output, target);
    }
}
