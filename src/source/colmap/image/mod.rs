//! COLMAP image module.

pub mod images;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use images::*;

use crate::function::{advance, is_null, read_bytes_before};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::{
    ffi::CString,
    io::{BufReader, BufWriter, Read, Write},
};

/// A COLMAP image.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Image {
    /// Image ID.
    pub image_id: u32,
    /// A normalized Hamiltonian quaternion.
    ///
    /// It represents the rotation from world space to view space.
    ///
    /// It is in **scalar-first** order, i.e., `[w, x, y, z]`.
    pub quaternion: [f64; 4],
    /// Translation in view space.
    pub translation: [f64; 3],
    /// [Camera ID](super::Camera::camera_id).
    pub camera_id: u32,
    /// Image file name.
    pub file_name: CString,
}

impl Image {
    /// Return the 3D position in world space.
    ///
    /// It takes the 3D rotation from world space to view space, i.e., [`Self::rotation`].
    ///
    /// # Formula
    ///
    /// Consider that 0 = [Self::rotation] * [Self::position] + [Self::translation].
    ///
    /// Therefore, [Self::position] = -[Self::rotation]^t * [Self::translation].
    pub const fn position(
        &self,
        rotation: &[[f64; 3]; 3],
    ) -> [f64; 3] {
        let r = rotation;
        let t = self.translation;
        [
            -r[0][0] * t[0] - r[0][1] * t[1] - r[0][2] * t[2],
            -r[1][0] * t[0] - r[1][1] * t[1] - r[1][2] * t[2],
            -r[2][0] * t[0] - r[2][1] * t[1] - r[2][2] * t[2],
        ]
    }

    /// Return the 3D rotation from world space to view space.
    ///
    /// It is in **column-major order**, i.e., `M[col][row]`.
    pub const fn rotation(&self) -> [[f64; 3]; 3] {
        let [w, x, y, z] = self.quaternion;
        let w_x = w * x * 2.0;
        let w_y = w * y * 2.0;
        let w_z = w * z * 2.0;
        let x_x = x * x * 2.0;
        let x_y = x * y * 2.0;
        let x_z = x * z * 2.0;
        let y_y = y * y * 2.0;
        let y_z = y * z * 2.0;
        let z_z = z * z * 2.0;
        [
            [1.0 - y_y - z_z, x_y + w_z, x_z - w_y],
            [x_y - w_z, 1.0 - x_x - z_z, y_z + w_x],
            [x_z + w_y, y_z - w_x, 1.0 - x_x - y_y],
        ]
    }
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

        let file_name = read_bytes_before(reader, is_null, 64)?;
        // SAFETY: The result of `read_bytes_before` never include the null terminator.
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
        writer.write_all(self.file_name.as_bytes_with_nul())?;

        // Write 0 to point count
        writer.write_u64::<LE>(0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn position() {
        use super::*;

        let image = Image {
            quaternion: [
                0.9928923624805012,
                0.006208227229002722,
                -0.11837120574960786,
                0.010699163142319695,
            ],
            translation: [2.1400970808418642, 0.18616441825409558, 4.726341984431894],
            ..Default::default()
        };
        let rotation = image.rotation();
        let position = image.position(&rotation);

        assert_eq!(
            position,
            [-3.194916373379071, -0.18378876753171225, -4.087996124741175]
        );
    }

    #[test]
    fn rotation() {
        use super::*;

        let image = Image {
            quaternion: [
                0.9898446088507,
                0.0712377208478,
                -0.122993928961,
                -0.002308873358,
            ],
            ..Default::default()
        };
        let rotation = image.rotation();

        assert_eq!(
            rotation,
            [
                [
                    0.9697343250851065000,
                    -0.022094466046466348,
                    0.2431607972553234400,
                ],
                [
                    -0.012952762662725100,
                    0.9898397124644553000,
                    0.1415965026675595000,
                ],
                [
                    -0.243818712758323920,
                    -0.140460593044464320,
                    0.9595953611342949000,
                ]
            ]
        );
    }
}
