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

pub fn read_to_image<R: io::Read + io::Seek>(
    reader: &mut R
) -> Result<image::RgbImage, Error> {
    image::io::Reader::new(io::BufReader::new(reader))
        .with_guessed_format()
        .map_err(Error::Io)?
        .decode()
        .map(image::DynamicImage::into_rgb8)
        .map_err(Error::Image)
}

macro_rules! read_to_slice {
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

pub(crate) use read_to_slice;

#[cfg(test)]
mod tests {
    #[test]
    fn read_to_image() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00,
            0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4, 0x89,
            0x00, 0x00, 0x00, 0x01, 0x73, 0x52, 0x47, 0x42, 0x00, 0xae, 0xce,
            0x1c, 0xe9, 0x00, 0x00, 0x00, 0x44, 0x65, 0x58, 0x49, 0x66, 0x4d,
            0x4d, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x87, 0x69,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1a, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x03, 0xa0, 0x01, 0x00, 0x03, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xa0, 0x02, 0x00, 0x04, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xa0, 0x03, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0xf9, 0x22, 0x9d, 0xfe, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44,
            0x41, 0x54, 0x08, 0x1d, 0x63, 0xf8, 0xcf, 0x60, 0xdb, 0x0d, 0x00,
            0x05, 0x06, 0x01, 0xc8, 0x5d, 0xd6, 0x92, 0xd1, 0x00, 0x00, 0x00,
            0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
        ]);

        let image = read_to_image(&mut reader);
        assert!(image.is_ok(), "{:#?}", image.unwrap_err());

        let image = image.unwrap();
        assert_eq!(image.height(), 1);
        assert_eq!(image.width(), 1);
        assert_eq!(image.get_pixel(0, 0).0, [0xff, 0x00, 0x3d]);
    }

    #[test]
    fn read_to_slice() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[1, 0, 0, 0, 4, 0, 0, 0]);
        let result = read_to_slice!(&mut reader, u32, 2);
        assert!(result.is_ok(), "{:#?}", result.unwrap_err());

        let result = result.unwrap();
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 4);
    }
}
