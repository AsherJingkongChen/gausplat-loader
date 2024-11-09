pub use crate::error::Error;
pub use bytemuck::Pod;

use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    fn decode(reader: &mut impl Read) -> Result<Self, Error>;
}

pub fn advance(
    reader: &mut impl Read,
    byte_count: usize,
) -> Result<(), Error> {
    // Using a buffer size of 8 KiB.
    const BUFFER_SIZE_LEVEL: usize = 3 + 10;
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;

    for _ in 0..(byte_count >> BUFFER_SIZE_LEVEL) {
        reader.read_exact(&mut [0; BUFFER_SIZE])?;
    }

    Ok(reader.read_exact(&mut vec![0; byte_count & (BUFFER_SIZE - 1)])?)
}

pub fn read_any<T: Pod>(reader: &mut impl Read) -> Result<T, Error> {
    let mut bytes = vec![0; std::mem::size_of::<T>()];
    reader.read_exact(&mut bytes)?;

    Ok(*bytemuck::from_bytes::<T>(&bytes))
}

pub fn read_string_until_zero(
    reader: &mut impl Read,
    reserved_size: usize,
) -> Result<String, Error> {
    let mut bytes = Vec::with_capacity(reserved_size);
    loop {
        let byte = &mut [0];
        let is_eof = reader.read(byte)? == 0;
        let byte = byte[0];
        if byte == 0 || is_eof {
            break;
        }
        bytes.push(byte);
    }

    Ok(String::from_utf8(bytes)?)
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
    fn read_string_until_zero() {
        use super::*;

        let source =
            include_bytes!("../../examples/data/hello-world/ascii+binary.dat");
        let reader = &mut std::io::Cursor::new(source);

        advance(reader, 8).unwrap();
        let target = "Hello, World!";
        let output = read_string_until_zero(reader, 16).unwrap();
        assert_eq!(output, target);

        let target = "Bonjour, le monde!\n";
        let output = read_string_until_zero(reader, 32).unwrap();
        assert_eq!(output, target);
    }
}
