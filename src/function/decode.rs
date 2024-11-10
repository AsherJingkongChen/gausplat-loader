pub use crate::error::Error;
pub use bytemuck::Pod;

use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    fn decode(reader: &mut impl Read) -> Result<Self, Error>;
}

/// Discarding `byte_count` bytes from the reader.
#[inline]
pub fn advance(
    reader: &mut impl Read,
    byte_count: usize,
) -> Result<(), Error> {
    // Using a buffer size of 8 KiB.
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;
    const BUFFER_SIZE_LEVEL: usize = 3 + 10;
    const BUFFER_SIZE_MASK: usize = BUFFER_SIZE - 1;

    for _ in 0..(byte_count >> BUFFER_SIZE_LEVEL) {
        reader.read_exact(&mut [0; BUFFER_SIZE])?;
    }

    Ok(reader.read_exact(&mut vec![0; byte_count & BUFFER_SIZE_MASK])?)
}

/// Reading any type of data from the reader.
#[inline]
pub fn read_any<T: Pod>(reader: &mut impl Read) -> Result<T, Error> {
    let bytes = &mut vec![0; std::mem::size_of::<T>()];
    reader.read_exact(bytes)?;

    Ok(*bytemuck::from_bytes::<T>(bytes))
}

/// Reading all bytes until the delimiter byte or EOF is reached.
pub fn read_byte_until(
    reader: &mut impl Read,
    delimiter: u8,
    reserved_size: usize,
) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::with_capacity(reserved_size.next_power_of_two());
    loop {
        let byte = &mut [0];
        let is_eof = reader.read(byte)? == 0;
        let byte = byte[0];
        if byte == delimiter || is_eof {
            break;
        }
        bytes.push(byte);
    }

    Ok(bytes)
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
    fn read_byte_until() {
        use super::*;

        let source =
            include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        advance(reader, 8).unwrap();
        let target = b"Hello, World!";
        let output = read_byte_until(reader, b'\0', target.len()).unwrap();
        assert_eq!(output, target);

        let target = b"Bonjour, le monde!\n";
        let output = read_byte_until(reader, b'\0', target.len()).unwrap();
        assert_eq!(output, target);
    }
}
