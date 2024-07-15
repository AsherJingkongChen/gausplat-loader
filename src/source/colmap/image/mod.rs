pub mod images;

pub use crate::function::Decoder;
use crate::{
    error::*,
    function::{advance, read_slice},
};
pub use images::*;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    image_id: u32,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    camera_id: u32,
    file_name: String,
}

impl Image {
    pub fn image_id(&self) -> &u32 {
        &self.image_id
    }

    pub fn camera_id(&self) -> &u32 {
        &self.camera_id
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

impl Decoder for Image {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let [image_id] = read_slice!(reader, u32, 1)?;
        let rotation = read_slice!(reader, f64, 4)?;
        let translation = read_slice!(reader, f64, 3)?;
        let [camera_id] = read_slice!(reader, u32, 1)?;
        let file_name = {
            let mut bytes = Vec::new();
            loop {
                let byte = read_slice!(reader, u8, 1)?[0];
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }
            String::from_utf8(bytes).map_err(Error::Utf8)?
        };
        {
            let point_count = read_slice!(reader, u64, 1)?[0] as usize;
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
