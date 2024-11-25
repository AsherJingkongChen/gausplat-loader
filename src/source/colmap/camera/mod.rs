pub mod cameras;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use cameras::*;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Camera {
    pub camera_id: u32,
    pub width: u64,
    pub height: u64,
    pub principal_point_x: f64,
    pub principal_point_y: f64,
    pub variant: CameraVariant,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraVariant {
    SimplePinhole {
        focal_length: f64,
    },
    Pinhole {
        focal_length_x: f64,
        focal_length_y: f64,
    },
    // TODO: Support more camera models from COLMAP. See https://github.com/colmap/colmap/blob/c238aec0e669610850badf3a3279dc2858f37f0f/src/colmap/sensor/models.h#L82
}

impl Camera {
    #[inline]
    pub fn model_id(&self) -> u32 {
        use CameraVariant::*;

        match self.variant {
            SimplePinhole { .. } => 0,
            Pinhole { .. } => 1,
        }
    }

    #[inline]
    pub fn focal_length_x(&self) -> f64 {
        use CameraVariant::*;

        match self.variant {
            SimplePinhole { focal_length } => focal_length,
            Pinhole { focal_length_x, .. } => focal_length_x,
        }
    }

    #[inline]
    pub fn focal_length_y(&self) -> f64 {
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
