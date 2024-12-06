//! COLMAP camera module.

pub mod cameras;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use cameras::*;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{BufReader, BufWriter, Read, Write};

/// A COLMAP camera.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Camera {
    /// Camera ID.
    pub camera_id: u32,
    /// Camera width.
    pub width: u64,
    /// Camera height.
    pub height: u64,
    /// Principal point x value.
    pub principal_point_x: f64,
    /// Principal point y value.
    pub principal_point_y: f64,
    /// Camera variant.
    pub variant: CameraVariant,
}

/// A COLMAP camera variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraVariant {
    /// Simple pinhole camera.
    SimplePinhole {
        /// Focal length.
        ///
        /// It is the same for x and y.
        focal_length: f64,
    },
    /// Pinhole camera.
    Pinhole {
        /// Focal length x value.
        focal_length_x: f64,
        /// Focal length y value.
        focal_length_y: f64,
    },
    // TODO: Support more camera models from COLMAP.
    // See https://github.com/colmap/colmap/blob/c238aec0e669610850badf3a3279dc2858f37f0f/src/colmap/sensor/models.h#L82
}

impl Camera {
    /// Return the model ID.
    #[inline]
    pub const fn model_id(&self) -> u32 {
        use CameraVariant::*;

        match self.variant {
            SimplePinhole { .. } => 0,
            Pinhole { .. } => 1,
        }
    }

    /// Return the focal length x value.
    #[inline]
    pub const fn focal_length_x(&self) -> f64 {
        use CameraVariant::*;

        match self.variant {
            SimplePinhole { focal_length } => focal_length,
            Pinhole { focal_length_x, .. } => focal_length_x,
        }
    }

    /// Return the focal length y value.
    #[inline]
    pub const fn focal_length_y(&self) -> f64 {
        use CameraVariant::*;

        match self.variant {
            SimplePinhole { focal_length } => focal_length,
            Pinhole { focal_length_y, .. } => focal_length_y,
        }
    }
}

impl Decoder for Camera {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use CameraVariant::*;

        let camera_id = reader.read_u32::<LE>()?;
        let model_id = reader.read_u32::<LE>()?;
        let width = reader.read_u64::<LE>()?;
        let height = reader.read_u64::<LE>()?;
        let variant = match model_id {
            0 => {
                let focal_length = reader.read_f64::<LE>()?;
                SimplePinhole { focal_length }
            },
            1 => {
                let focal_length_x = reader.read_f64::<LE>()?;
                let focal_length_y = reader.read_f64::<LE>()?;
                Pinhole {
                    focal_length_x,
                    focal_length_y,
                }
            },
            _ => return Err(Error::InvalidCameraModelId(model_id)),
        };
        let principal_point_x = reader.read_f64::<LE>()?;
        let principal_point_y = reader.read_f64::<LE>()?;

        Ok(Self {
            camera_id,
            width,
            height,
            principal_point_x,
            principal_point_y,
            variant,
        })
    }
}

impl Encoder for Camera {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        use CameraVariant::*;

        writer.write_u32::<LE>(self.camera_id)?;
        writer.write_u32::<LE>(self.model_id())?;
        writer.write_u64::<LE>(self.width)?;
        writer.write_u64::<LE>(self.height)?;
        match self.variant {
            SimplePinhole { focal_length } => writer.write_f64::<LE>(focal_length),
            Pinhole {
                focal_length_x,
                focal_length_y,
            } => {
                writer.write_f64::<LE>(focal_length_x)?;
                writer.write_f64::<LE>(focal_length_y)
            },
        }?;
        writer.write_f64::<LE>(self.principal_point_x)?;
        writer.write_f64::<LE>(self.principal_point_y)?;

        Ok(())
    }
}

impl Default for CameraVariant {
    #[inline]
    fn default() -> Self {
        CameraVariant::SimplePinhole {
            focal_length: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default() {
        use super::*;

        let target = CameraVariant::SimplePinhole { focal_length: 0.0 };
        let output = Camera::default().variant;
        assert_eq!(output, target);
    }
}
