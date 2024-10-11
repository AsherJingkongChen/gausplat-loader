pub mod cameras;
pub mod pinhole;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use cameras::*;
pub use pinhole::*;

use crate::function::{advance, read_slice};
use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Camera {
    Pinhole(PinholeCamera),
}

impl Camera {
    pub fn camera_id(&self) -> u32 {
        match self {
            Self::Pinhole(camera) => camera.camera_id,
        }
    }
}

impl Decoder for Camera {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let [camera_id, model_id] = read_slice::<u32, 2>(reader)?;
        let [width, height] = read_slice::<u64, 2>(reader)?;

        match model_id {
            0..2 => {
                let [focal_length_x, focal_length_y] = match model_id {
                    0 => {
                        let [focal_length] = read_slice::<f64, 1>(reader)?;
                        [focal_length, focal_length]
                    },
                    1 => read_slice::<f64, 2>(reader)?,
                    _ => unreachable!(),
                };
                advance(reader, 16)?;
                Ok(Self::Pinhole(PinholeCamera {
                    camera_id,
                    width,
                    height,
                    focal_length_x,
                    focal_length_y,
                }))
            },
            _ => Err(Error::UnknownCameraModelId(model_id)),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::Pinhole(Default::default())
    }
}
