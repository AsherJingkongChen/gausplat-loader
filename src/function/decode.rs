pub use super::{NULL, SPACE};
pub use crate::error::Error;

use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    type Err;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err>;
}

/// Discarding `n` bytes.
#[inline]
pub fn advance(
    reader: &mut impl Read,
    n: usize,
) -> Result<(), Error> {
    // Using a cache size of 128 bytes.
    const CACHE_SIZE: usize = 1 << CACHE_SIZE_LEVEL;
    const CACHE_SIZE_LEVEL: usize = 7;
    const CACHE_SIZE_MASK: usize = CACHE_SIZE - 1;
    let cache = &mut [0; CACHE_SIZE];

    (0..n >> CACHE_SIZE_LEVEL)
        .try_for_each(|_| reader.read_exact(cache))
        .and_then(|_| reader.read_exact(&mut cache[..n & CACHE_SIZE_MASK]))
        .map_err(Into::into)
}

/// Checking if the byte is [`NULL[0]`](NULL).
#[inline]
pub const fn is_null(byte: u8) -> bool {
    byte == NULL[0]
}

/// Checking if the byte is [`SPACE[0]`](SPACE).
#[inline]
pub const fn is_space(byte: u8) -> bool {
    byte == SPACE[0]
}

/// Reading `n` bytes.
#[inline]
pub fn read_bytes(
    reader: &mut impl Read,
    n: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![0; n];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

/// Reading a byte after all delimiter bytes or `None` at EOF.
pub fn read_byte_after(
    reader: &mut impl Read,
    delimiter: impl Fn(u8) -> bool,
) -> Result<Option<u8>, Error> {
    loop {
        let byte = &mut [0; 1];
        let is_eof = reader.read(byte)? == 0;
        if is_eof {
            return Ok(None);
        }
        if !delimiter(byte[0]) {
            return Ok(Some(byte[0]));
        }
    }
}

/// Reading all bytes before the delimiter or EOF.
#[inline]
pub fn read_bytes_before(
    reader: &mut impl Read,
    delimiter: impl Fn(u8) -> bool,
    capacity: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::with_capacity(capacity);
    loop {
        let byte = &mut [0; 1];
        let is_eof = reader.read(byte)? == 0;
        if is_eof || delimiter(byte[0]) {
            return Ok(bytes);
        }
        bytes.push(byte[0]);
    }
}

/// Reading `N` bytes.
#[inline]
pub fn read_bytes_const<const N: usize>(
    reader: &mut impl Read
) -> Result<[u8; N], Error> {
    let mut bytes = [0; N];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

/// Reading all bytes before the delimiters or EOF.
pub fn read_bytes_before_many_const<const N: usize>(
    reader: &mut impl Read,
    delimiters: &[u8; N],
    capacity: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::with_capacity(capacity);
    let mut ring = [0; N];
    let mut pos = 0;

    loop {
        let byte = &mut [0; 1];
        let is_eof = reader.read(byte)? == 0;
        if is_eof {
            return Ok(bytes);
        }

        bytes.push(byte[0]);
        ring[pos % N] = byte[0];
        pos += 1;
        if pos >= N && (0..N).all(|idx| ring[(idx + pos) % N] == delimiters[idx]) {
            bytes.truncate(pos - N);
            return Ok(bytes);
        }
    }
}

/// Reading all bytes before the CRLF, LF, or EOF.
#[inline]
pub fn read_bytes_before_newline(
    reader: &mut impl Read,
    capacity: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::with_capacity(capacity);
    loop {
        let byte = &mut [0; 1];
        let is_eof = reader.read(byte)? == 0;
        if is_eof || byte[0] == b'\n' {
            return Ok(bytes);
        }
        if byte[0] == b'\r' {
            let is_eof = reader.read(byte)? == 0;
            if is_eof || byte[0] == b'\n' {
                return Ok(bytes);
            }
            bytes.push(b'\r');
        }
        bytes.push(byte[0]);
    }
}

/// Reading exact one CRLF or LF.
#[inline]
pub fn read_newline(reader: &mut impl Read) -> Result<Box<[u8]>, Error> {
    let mut byte = [0; 1];
    reader.read_exact(&mut byte)?;
    if byte[0] == b'\n' {
        return Ok([b'\n'].into());
    }
    if byte[0] == b'\r' {
        reader.read_exact(&mut byte)?;
        if byte[0] == b'\n' {
            return Ok([b'\r', b'\n'].into());
        }
    }
    Err(Error::MissingToken("<newline>".into()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn advance() {
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(b"\x01\x02\0\0\x04\0\x50\0");

        advance(reader, 4).unwrap();
        let output = read_bytes_const(reader).unwrap();
        let target = [0x04, 0x00, 0x50, 0x00];
        assert_eq!(output, target);

        advance(reader, 4).unwrap_err();
    }

    #[test]
    fn read_bytes() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut Cursor::new(source);

        let target = &source[0..24];
        let output = read_bytes(reader, 24).unwrap();
        assert_eq!(output, target);

        let target = &source[24..40];
        let output = read_bytes(reader, 16).unwrap();
        assert_eq!(output, target);

        read_bytes(reader, 2).unwrap_err();

        let target = std::io::ErrorKind::UnexpectedEof;
        let output = reader.read_exact(&mut [0; 1]).unwrap_err().kind();
        assert_eq!(output, target);

        read_bytes(reader, 1).unwrap_err();
    }

    #[test]
    fn read_byte_after() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!("../../examples/data/hello-world/ascii+space.txt");
        let reader = &mut Cursor::new(source);

        let target = Some(b',');
        let output = read_byte_after(reader, |b| b" Helo".contains(&b)).unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = read_byte_after(&mut Cursor::new([]), is_space).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_bytes_before() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        advance(reader, 8).unwrap();
        let target = b"Hello, World!";
        let output = read_bytes_before(reader, |b| b == 0, 16).unwrap();
        assert_eq!(output, target);

        let target = b"Bonjour, le monde!\n";
        let output = read_bytes_before(reader, |b| b == 0, 16).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_bytes_const() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut Cursor::new(source);

        let target = [0xd5, 0xda, 0x34, 0x01, 0x60, 0xcc, 0xd5, 0x07];
        let output = read_bytes_const(reader).unwrap();
        assert_eq!(output, target);

        read_bytes_const::<512>(reader).unwrap_err();
    }

    #[test]
    fn read_bytes_after_and_before() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/ascii+space.txt");
        let reader = &mut std::io::Cursor::new(source);

        let target = Some(b'H');
        let output = read_byte_after(reader, is_space).unwrap();
        assert_eq!(output, target);

        let target = b"ello, World!";
        let output = read_bytes_before(reader, |b| b == b'\n', 16).unwrap();
        assert_eq!(output, target);

        let target = Some(b'B');
        let output = read_byte_after(reader, is_space).unwrap();
        assert_eq!(output, target);

        let target = b"onjour, le monde";
        let output = read_bytes_before(reader, |b| b == b'!', 16).unwrap();
        assert_eq!(output, target);

        let target = Some(b'\n');
        let output = read_byte_after(reader, is_space).unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = read_byte_after(reader, is_space).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_bytes_before_many_const() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/ascii+space.txt");
        let reader = &mut std::io::Cursor::new(source);

        advance(reader, 12).unwrap();
        let target = b"Hello";
        let output = &read_bytes_before_many_const(reader, b", ", 16).unwrap();
        assert_eq!(output, target);

        advance(reader, 19).unwrap();
        let target = b"Bonjour, le";
        let output = &read_bytes_before_many_const(reader, b" monde", 20).unwrap();
        assert_eq!(output, target);

        read_bytes_before_many_const(reader, b" monde", 20).unwrap();
    }

    #[test]
    fn read_bytes_before_newline() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/utf8+newline.txt");
        let reader = &mut std::io::Cursor::new(source);

        let target = b"Hello, World!";
        let output = read_bytes_before_newline(reader, 16).unwrap();
        assert_eq!(output, target);

        let target = "\u{4f60}\u{597d}\u{ff0c}".as_bytes();
        let output = read_bytes_before_newline(reader, 8).unwrap();
        assert_eq!(output, target);

        let target = "\u{4e16}\u{754c}\u{ff01} ".as_bytes();
        let output = read_bytes_before_newline(reader, 8).unwrap();
        assert_eq!(output, target);

        // NOTE: In some viewers, a carriage return (CR) is displayed as a newline.
        // However, it is not considered a newline in this function.
        let target = b"";
        let output = read_bytes_before_newline(reader, 4).unwrap();
        assert_eq!(output, target);
        let output = read_bytes_before_newline(reader, 4).unwrap();
        assert_eq!(output, target);

        let target = b"\rBonjour, le monde!  ";
        let output = read_bytes_before_newline(reader, 20).unwrap();
        assert_eq!(output, target);

        let target = std::io::ErrorKind::UnexpectedEof;
        let output = reader.read_exact(&mut [0; 1]).unwrap_err().kind();
        assert_eq!(output, target);

        let target = b"";
        let output = read_bytes_before_newline(reader, 4).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_newline() {
        use super::*;

        let source = b"\nHi!";
        let reader = &mut std::io::Cursor::new(source);
        let target = (*b"\n").into();
        let output = read_newline(reader).unwrap();
        assert_eq!(output, target);

        let source = b"\r\nHi!";
        let reader = &mut std::io::Cursor::new(source);
        let target = (*b"\r\n").into();
        let output = read_newline(reader).unwrap();
        assert_eq!(output, target);

        let source = b"\rHi!";
        let reader = &mut std::io::Cursor::new(source);
        read_newline(reader).unwrap_err();

        let source = b"\n\nHi!";
        let reader = &mut std::io::Cursor::new(source);
        let target = (*b"\n").into();
        let output = read_newline(reader).unwrap();
        assert_eq!(output, target);
    }
}
