use crate::error::*;
use std::io;

pub trait Decoder
where
    Self: Sized,
{
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

pub(crate) fn advance<R: io::Read>(
    reader: &mut R,
    byte_count: usize,
) -> Result<(), DecodeError> {
    const BUFFER_SIZE_LEVEL: usize = 3 + 10;
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;

    for _ in 0..(byte_count >> BUFFER_SIZE_LEVEL) {
        reader
            .read_exact(&mut [0; BUFFER_SIZE])
            .map_err(DecodeError::Io)?;
    }
    reader
        .read_exact(&mut vec![0; byte_count & (BUFFER_SIZE - 1)])
        .map_err(DecodeError::Io)
}

macro_rules! read_to_slice {
    ($R:expr, $T:ty, $N:expr) => {{
        use crate::error::DecodeError;

        let mut bytes = [0; $N * std::mem::size_of::<$T>()];

        std::io::Read::read_exact($R, &mut bytes)
            .map_err(DecodeError::Io)
            .and_then(|_| {
                bytemuck::checked::try_from_bytes::<[$T; $N]>(&bytes)
                    .map_err(DecodeError::Cast)
                    .map(|v| *v)
            })
    }};
}

pub(crate) use read_to_slice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_to_slice() {
        let mut reader = std::io::Cursor::new(&[1, 0, 0, 0, 4, 0, 0, 0]);
        let result = read_to_slice!(&mut reader, u32, 2);
        assert!(result.is_ok(), "{:#?}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 4);
    }
}
