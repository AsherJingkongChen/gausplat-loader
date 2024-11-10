pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fmt, fs,
    io::{BufReader, BufWriter, Read, Seek, Write},
    path::{Path, PathBuf},
};

/// Duplex file stream.
#[derive(Clone, PartialEq)]
pub struct File<S> {
    pub inner: S,
    pub path: PathBuf,
}

impl<R: Read> File<R> {
    pub fn read(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        let reader = &mut BufReader::new(&mut self.inner);
        reader.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

impl<W: Write> File<W> {
    pub fn write(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), Error> {
        let writer = &mut BufWriter::new(&mut self.inner);
        writer.write_all(bytes)?;

        Ok(())
    }
}

impl<S: Seek> File<S> {
    pub fn rewind(&mut self) -> Result<(), Error> {
        Ok(self.inner.rewind()?)
    }
}

impl fmt::Debug for File<fs::File> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl<R: Default> Default for File<R> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            path: Default::default(),
        }
    }
}

impl Opener for File<fs::File> {
    fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let inner = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .truncate(false)
            .write(true)
            .open(&path)?;
        let path = path.as_ref().to_owned();

        Ok(Self { inner, path })
    }
}

impl<R: Read> Read for File<R> {
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<S: Seek> Seek for File<S> {
    fn seek(
        &mut self,
        pos: std::io::SeekFrom,
    ) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl<W: Write> Write for File<W> {
    fn write(
        &mut self,
        buf: &[u8],
    ) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
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
            path: Default::default(),
            inner: std::io::Cursor::new(source),
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
        let output = file.inner.into_inner();
        assert_eq!(output, target);
    }
}
