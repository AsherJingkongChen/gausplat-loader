pub mod images;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use images::*;

use crate::function::{advance, read_bytes_before, write_bytes};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::{
    ffi::CString,
    io::{BufReader, BufWriter, Read, Write},
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
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let image_id = reader.read_u32::<LE>()?;
        let quaternion = [
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
        ];
        let translation = [
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
        ];
        let camera_id = reader.read_u32::<LE>()?;

        let file_name = read_bytes_before(reader, |b| b == 0, 64)?;
        // SAFETY: The result of `read_bytes_before` does not include the null terminator.
        let file_name = unsafe { CString::from_vec_unchecked(file_name) };

        // Skip points
        let point_count = reader.read_u64::<LE>()? as usize;
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
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_u32::<LE>(self.image_id)?;
        writer.write_f64::<LE>(self.quaternion[0])?;
        writer.write_f64::<LE>(self.quaternion[1])?;
        writer.write_f64::<LE>(self.quaternion[2])?;
        writer.write_f64::<LE>(self.quaternion[3])?;
        writer.write_f64::<LE>(self.translation[0])?;
        writer.write_f64::<LE>(self.translation[1])?;
        writer.write_f64::<LE>(self.translation[2])?;
        writer.write_u32::<LE>(self.camera_id)?;
        write_bytes(writer, self.file_name.as_bytes_with_nul())?;

        // Write 0 to point count
        writer.write_u64::<LE>(0)?;

        Ok(())
    }
}
