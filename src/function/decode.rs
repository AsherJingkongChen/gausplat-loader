use crate::error::*;
use std::io;

pub trait Decoder
where
    Self: Sized,
{
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error>;
}

pub(crate) fn advance<R: io::Read>(
    reader: &mut R,
    byte_count: usize,
) -> Result<(), Error> {
    const BUFFER_SIZE_LEVEL: usize = 3 + 10;
    const BUFFER_SIZE: usize = 1 << BUFFER_SIZE_LEVEL;

    for _ in 0..(byte_count >> BUFFER_SIZE_LEVEL) {
        reader
            .read_exact(&mut [0; BUFFER_SIZE])
            .map_err(Error::Io)?;
    }
    reader
        .read_exact(&mut vec![0; byte_count & (BUFFER_SIZE - 1)])
        .map_err(Error::Io)
}

macro_rules! read_slice {
    ($R:expr, $T:ty, $N:expr) => {{
        use crate::error::Error;

        let mut bytes = [0; $N * std::mem::size_of::<$T>()];

        std::io::Read::read_exact($R, &mut bytes)
            .map_err(Error::Io)
            .and_then(|_| {
                bytemuck::checked::try_from_bytes::<[$T; $N]>(&bytes)
                    .map_err(Error::Cast)
                    .map(|v| *v)
            })
    }};
}

pub(crate) use read_slice;

#[cfg(test)]
mod tests {
    #[test]
    fn read_slice() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[1, 0, 0, 0, 4, 0, 0, 0]);
        let result = read_slice!(&mut reader, u32, 2);
        assert!(result.is_ok(), "{:#?}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 4);
    }
}
