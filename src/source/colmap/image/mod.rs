pub mod images;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use images::*;

use crate::function::{advance, read_any, read_string_until_zero};
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
        let image_id = read_any::<u32>(reader)?;
        let quaternion = read_any::<[f64; 4]>(reader)?;
        let translation = read_any::<[f64; 3]>(reader)?;
        let camera_id = read_any::<u32>(reader)?;
        let file_name = read_string_until_zero(reader, 64)?;
        let point_count = read_any::<u64>(reader)? as usize;
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
