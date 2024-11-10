pub mod images;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use images::*;

use crate::function::{
    advance, read_any, read_bytes_before, write_any, write_bytes,
};
use std::{
    ffi::CString,
    io::{Read, Write},
};

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
    pub file_name: CString,
}

impl Decoder for Image {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let image_id = read_any::<u32>(reader)?;
        let quaternion = read_any::<[f64; 4]>(reader)?;
        let translation = read_any::<[f64; 3]>(reader)?;
        let camera_id = read_any::<u32>(reader)?;
        let file_name = unsafe {
            // SAFETY: `read_bytes_before` does not include the null terminator.
            CString::from_vec_unchecked(read_bytes_before(reader, b'\0', 64)?)
        };
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
        write_bytes(writer, self.file_name.as_bytes_with_nul())?;
        // Write 0 to point count
        write_any(writer, &0_u64)?;

        Ok(())
    }
}
