pub mod images;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use images::*;

use crate::function::{
    advance, read_any, read_byte_until, write_any, write_str,
};
use std::io::{Read, Write};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Image {
    pub image_id: u32,

    /// A normalized Hamiltonian quaternion
    /// **(in scalar-first order, i.e., `[w, x, y, z]`)**.
    ///
    /// It represents the rotation from world space to view space.
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
        let file_name = String::from_utf8(read_byte_until(reader, b'\0', 64)?)?;
        let point_count = read_any::<u64>(reader)? as usize;
        // Skip points
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

impl Encoder for Image {
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error> {
        write_any(writer, &self.image_id)?;
        write_any(writer, &self.quaternion)?;
        write_any(writer, &self.translation)?;
        write_any(writer, &self.camera_id)?;
        write_str(writer, &self.file_name)?;
        write_any(writer, &b'\0')?;
        // Write 0 to point count
        write_any(writer, &0_u64)?;

        Ok(())
    }
}
