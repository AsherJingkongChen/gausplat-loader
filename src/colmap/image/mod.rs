pub mod images;

pub use crate::function::Decoder;
use crate::{error::*, function::read_to_slice};
use std::io::{self, SeekFrom};

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub image_id: u32,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    pub camera_id: u32,
    pub file_name: String,
}

impl Decoder for Image {
    fn decode<R: io::BufRead + io::Seek>(
        reader: &mut R
    ) -> Result<Self, DecodeError> {
        let [image_id] = read_to_slice!(reader, u32, 1)?;
        let rotation = read_to_slice!(reader, f64, 4)?;
        let translation = read_to_slice!(reader, f64, 3)?;
        let [camera_id] = read_to_slice!(reader, u32, 1)?;
        let file_name = {
            let mut bytes = Vec::new();
            reader.read_until(0, &mut bytes).map_err(DecodeError::Io)?;
            bytes.pop();
            String::from_utf8(bytes).map_err(DecodeError::Utf8)?
        };
        {
            let point_count = read_to_slice!(reader, u64, 1)?[0] as i64;
            reader
                .seek(SeekFrom::Current(24 * point_count))
                .map_err(DecodeError::Io)?;
        };

        Ok(Self {
            image_id,
            rotation,
            translation,
            camera_id,
            file_name,
        })
    }
}
