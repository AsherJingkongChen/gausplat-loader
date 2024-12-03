pub mod files;

pub use crate::error::Error;
pub use crate::function::Opener;
pub use files::*;

use std::{
    fs,
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

/// Duplex file stream.
#[derive(Clone, Debug, PartialEq)]
pub struct File<F> {
    pub inner: F,
    pub path: PathBuf,
}

impl File<fs::File> {
    #[inline]
    pub fn truncate(&mut self) -> Result<&mut Self, Error> {
        self.inner.set_len(0)?;
        Ok(self)
    }
}

impl<R: Read> File<R> {
    #[inline]
    pub fn read_all(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        BufReader::new(&mut self.inner).read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

impl<W: Write> File<W> {
    #[inline]
    pub fn write_all(
        &mut self,
        bytes: &[u8],
    ) -> Result<&mut Self, Error> {
        BufWriter::new(&mut self.inner).write_all(bytes)?;
        Ok(self)
    }
}

impl<R: Read> Read for File<R> {
    #[inline]
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<S: Seek> Seek for File<S> {
    #[inline]
    fn seek(
        &mut self,
        pos: io::SeekFrom,
    ) -> io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl<W: Write> Write for File<W> {
    #[inline]
    fn write(
        &mut self,
        buf: &[u8],
    ) -> io::Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<F: Default> Default for File<F> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
            path: Default::default(),
        }
    }
}

impl<F> Deref for File<F> {
    type Target = F;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<F> DerefMut for File<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Opener for File<fs::File> {
    /// The file is opened in read and write mode.
    ///
    /// This won't truncate the previous file.
    /// One should call [`File::truncate`] to do so.
    #[inline]
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
    fn open_and_read_all() {
        use super::*;

        let source = "examples/data/hello-world/ascii.txt";
        let mut file = File::open(source).unwrap();

        let target = b"Hello, World!";
        let output = file.read_all().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn open_on_symlink() {
        use super::*;

        let source = "examples/data/hello-world.symlink/ascii.symlink.txt";
        let mut file = File::open(source).unwrap();

        let target = b"Hello, World!";
        let output = file.read_all().unwrap();
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

        let source = &include_bytes!("../../../examples/data/hello-world/ascii.txt")[..];
        let mut file = File {
            path: Default::default(),
            inner: std::io::Cursor::new(source),
        };

        let target = source;
        let output = file.read_all().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn write_and_rewind() {
        use super::*;

        let source = &include_bytes!("../../../examples/data/hello-world/ascii.txt")[..];
        let mut file = File::<std::io::Cursor<Vec<u8>>>::default();

        let target = source;
        file.write_all(source).unwrap();
        file.rewind().unwrap();
        let output = file.deref().to_owned().into_inner();
        assert_eq!(output, target);
        let output = file.deref_mut().to_owned().into_inner();
        assert_eq!(output, target);
    }
}
