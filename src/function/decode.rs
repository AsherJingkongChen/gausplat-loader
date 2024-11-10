pub use crate::error::Error;
pub use bytemuck::Pod;

use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    fn decode(reader: &mut impl Read) -> Result<Self, Error>;
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

/// Reading any type of data.
#[inline]
pub fn read_any<T: Pod>(reader: &mut impl Read) -> Result<T, Error> {
    let bytes = &mut vec![0; std::mem::size_of::<T>()];
    reader.read_exact(bytes)?;

    Ok(*bytemuck::from_bytes::<T>(bytes))
}

/// Reading a byte after all delimiter bytes or `None` at EOF.
pub fn read_byte_after(
    reader: &mut impl Read,
    delimiter: u8,
) -> Result<Option<u8>, Error> {
    let byte = &mut [0];
    loop {
        let is_eof = reader.read(byte)? == 0;
        let byte = byte[0];
        if byte != delimiter {
            return Ok(Some(byte));
        } else if is_eof {
            return Ok(None);
        }
    }
}

/// Reads all bytes before the delimiter or EOF.
#[inline]
pub fn read_bytes_before(
    reader: &mut impl Read,
    delimiter: u8,
    capacity: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::with_capacity(capacity);
    let byte = &mut [0];
    loop {
        let is_eof = reader.read(byte)? == 0;
        let byte = byte[0];
        if byte == delimiter || is_eof {
            return Ok(bytes);
        }
        bytes.push(byte);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn advance() {
        use super::*;

        let reader = &mut std::io::Cursor::new(&[
            0x01, 0x02, 0x00, 0x00, 0x04, 0x00, 0x50, 0x00,
        ]);

        advance(reader, 4).unwrap();
        let output = read_any::<u32>(reader).unwrap();
        let target = 0x00500004;
        assert_eq!(output, target);

        advance(reader, 4).unwrap_err();
    }

    #[test]
    fn read_any() {
        use super::*;

        let source =
            include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        let target = [20241109, 131452000];
        let output = read_any::<[u32; 2]>(reader).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_byte_after_and_before() {
        use super::*;

        let source =
            include_bytes!("../../examples/data/hello-world/ascii+space.txt");
        let reader = &mut std::io::Cursor::new(source);

        let target = Some(b'H');
        let output = read_byte_after(reader, b' ').unwrap();
        assert_eq!(output, target);

        let target = b"ello, World!";
        let output = read_bytes_before(reader, b'\n', 64).unwrap();
        assert_eq!(output, target);

        let target = Some(b'B');
        let output = read_byte_after(reader, b' ').unwrap();
        assert_eq!(output, target);

        let target = b"onjour, le monde";
        let output = read_bytes_before(reader, b'!', 64).unwrap();
        assert_eq!(output, target);

        let target = Some(b'\n');
        let output = read_byte_after(reader, b' ').unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = read_byte_after(reader, b' ').unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn read_bytes_before() {
        use super::*;

        let source =
            include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        advance(reader, 8).unwrap();
        let target = b"Hello, World!";
        let output = read_bytes_before(reader, b'\0', 64).unwrap();
        assert_eq!(output, target);

        let target = b"Bonjour, le monde!\n";
        let output = read_bytes_before(reader, b'\0', 64).unwrap();
        assert_eq!(output, target);
    }
}
