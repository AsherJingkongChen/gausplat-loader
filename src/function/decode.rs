pub use crate::error::Error;

use super::SPACE;
use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    type Err;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err>;
}

/// Discarding `size` bytes.
#[inline]
pub fn advance(
    reader: &mut impl Read,
    size: usize,
) -> Result<(), Error> {
    // Using a buffer size of 16 KiB.
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;
    const BUFFER_SIZE_LEVEL: usize = 4 + 10;
    const BUFFER_SIZE_MASK: usize = BUFFER_SIZE - 1;

    for _ in 0..(size >> BUFFER_SIZE_LEVEL) {
        reader.read_exact(&mut [0; BUFFER_SIZE])?;
    }

    Ok(reader.read_exact(&mut vec![0; size & BUFFER_SIZE_MASK])?)
}

#[inline]
pub const fn is_space(byte: u8) -> bool {
    byte == SPACE[0]
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
pub fn read_byte_after<DF: Fn(u8) -> bool>(
    reader: &mut impl Read,
    delimiter: DF,
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

/// Reads all bytes before the delimiter or EOF.
#[inline]
pub fn read_bytes_before<DF: Fn(u8) -> bool>(
    reader: &mut impl Read,
    delimiter: DF,
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

/// Reads all bytes before the CRLF, LF, or EOF.
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

#[cfg(test)]
mod tests {
    #[test]
    fn advance() {
        use super::*;

        let reader =
            &mut std::io::Cursor::new(&[0x01, 0x02, 0x00, 0x00, 0x04, 0x00, 0x50, 0x00]);

        advance(reader, 4).unwrap();
        let output = read_bytes_const(reader).unwrap();
        let target = [0x04, 0x00, 0x50, 0x00];
        assert_eq!(output, target);

        advance(reader, 4).unwrap_err();
    }

    #[test]
    fn read_bytes_const() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        let target = [0xd5, 0xda, 0x34, 0x01, 0x60, 0xcc, 0xd5, 0x07];
        let output = read_bytes_const(reader).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_bytes() {
        use super::*;

        let source = include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

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

        let source = include_bytes!("../../examples/data/hello-world/ascii+space.txt");
        let reader = &mut std::io::Cursor::new(source);

        let target = Some(b',');
        let output = read_byte_after(reader, |b| b" Helo".contains(&b)).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_byte_after_and_before() {
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
}
