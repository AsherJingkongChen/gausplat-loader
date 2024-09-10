pub mod images;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use images::*;

use crate::function::{advance, read_slice};
use std::io::Read;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Image {
    pub image_id: u32,
    pub quaternion: [f64; 4],
    pub translation: [f64; 3],
    pub camera_id: u32,
    pub file_name: String,
}

impl Decoder for Image {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let [image_id] = read_slice::<u32, 1>(reader)?;
        let quaternion = read_slice::<f64, 4>(reader)?;
        let translation = read_slice::<f64, 3>(reader)?;
        let [camera_id] = read_slice::<u32, 1>(reader)?;
        let file_name = {
            let mut bytes = Vec::with_capacity(16);
            loop {
                let [byte] = read_slice::<u8, 1>(reader)?;
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }
            String::from_utf8(bytes)?
        };
        let point_count = read_slice::<u64, 1>(reader)?[0] as usize;
        advance(reader, 24 * point_count)?;

        Ok(Self {
            image_id,
            quaternion,
            translation,
            camera_id,
            file_name,
        })
    }
}
