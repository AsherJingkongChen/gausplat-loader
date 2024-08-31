pub use crate::error::Error;

use std::io::Read;

pub trait Decoder
where
    Self: Sized,
{
    fn decode(reader: &mut impl Read) -> Result<Self, Error>;
}

pub(crate) fn advance(
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

pub(crate) fn read_slice<T, const N: usize>(
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
    fn read_slice() {
        use super::*;

        let reader = &mut std::io::Cursor::new(&[1, 0, 0, 0, 4, 0, 0, 0]);
        let result = read_slice::<u32, 2>(reader);
        assert!(result.is_ok(), "{}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 4);
    }
}
