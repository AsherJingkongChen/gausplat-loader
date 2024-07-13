pub mod images;

pub use crate::function::Decoder;
use crate::{
    error::*,
    function::{advance, read_to_slice},
};
pub use images::*;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub image_id: u32,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    pub camera_id: u32,
    pub file_name: String,
}

impl Decoder for Image {
    fn decode<R: io::Read + io::Seek>(
        reader: &mut R
    ) -> Result<Self, DecodeError> {
        let reader = &mut io::BufReader::new(reader);
        let [image_id] = read_to_slice!(reader, u32, 1)?;
        let rotation = read_to_slice!(reader, f64, 4)?;
        let translation = read_to_slice!(reader, f64, 3)?;
        let [camera_id] = read_to_slice!(reader, u32, 1)?;
        let file_name = {
            use io::BufRead;

            let mut bytes = Vec::new();
            reader.read_until(0, &mut bytes).map_err(DecodeError::Io)?;
            bytes.pop();
            String::from_utf8(bytes).map_err(DecodeError::Utf8)?
        };
        {
            let point_count = read_to_slice!(reader, u64, 1)?[0];
            advance(reader, 24 * point_count)?;
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
