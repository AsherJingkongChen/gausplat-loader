pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fs,
    io::{BufReader, BufWriter, Read, Seek, Write},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

/// Duplex file stream.
#[derive(Clone, Debug, PartialEq)]
pub struct File<F> {
    pub inner: F,
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

impl<F: Default> Default for File<F> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            path: Default::default(),
        }
    }
}

impl<F> Deref for File<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<F> DerefMut for File<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
    fn open_on_symlink() {
        use super::*;

        let source = "examples/data/hello-world.symlink/ascii.symlink.txt";
        let mut file = File::open(source).unwrap();

        let target = b"Hello, World!";
        let output = file.read().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn open_on_directory() {
        use super::*;

        let source = "examples/data/hello-world/";
        File::open(source).unwrap_err();
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
        let output = (*file).to_owned().into_inner();
        assert_eq!(output, target);
        let output = (&mut *file).to_owned().into_inner();
        assert_eq!(output, target);
    }
}
