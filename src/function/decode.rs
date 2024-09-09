pub use crate::error::Error;

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
    const BUFFER_SIZE_LEVEL: usize = 3 + 10;
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;

    for _ in 0..(byte_count >> BUFFER_SIZE_LEVEL) {
        reader.read_exact(&mut [0; BUFFER_SIZE])?;
    }

    Ok(reader.read_exact(&mut vec![0; byte_count & (BUFFER_SIZE - 1)])?)
}

pub fn read_slice<T, const N: usize>(
    reader: &mut impl Read
) -> Result<[T; N], Error>
where
    [T; N]: bytemuck::Pod,
{
    let mut bytes = vec![0; std::mem::size_of::<[T; N]>()];
    reader.read_exact(&mut bytes)?;

    Ok(bytemuck::from_bytes::<[T; N]>(&bytes).to_owned())
}

#[cfg(test)]
mod tests {
    #[test]
    fn advance() {
        use super::*;

        let reader = &mut std::io::Cursor::new(&[
            0x01, 0x02, 0x00, 0x00, 0x04, 0x00, 0x50, 0x00,
        ]);
        let result = advance(reader, 4);
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let result = read_slice::<u32, 1>(reader);
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 0x00500004);

        let result = advance(reader, 4);
        assert!(result.is_err());
    }

    #[test]
    fn read_slice() {
        use super::*;

        let reader = &mut std::io::Cursor::new(&[
            0x01, 0x02, 0x00, 0x00, 0x04, 0x00, 0x50, 0x00,
        ]);
        let result = read_slice::<u32, 2>(reader);
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 0x00000201);
        assert_eq!(result[1], 0x00500004);
    }
}
