pub mod cameras;
pub mod pinhole;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use cameras::*;
pub use pinhole::*;

use crate::function::read_any;
use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Camera {
    Pinhole(PinholeCamera),
    // TODO: Support more camera models from COLMAP.
}

impl Camera {
    pub fn camera_id(&self) -> u32 {
        match self {
            Self::Pinhole(camera) => camera.camera_id,
        }
    }

    pub fn height(&self) -> u64 {
        match self {
            Self::Pinhole(camera) => camera.height,
        }
    }

    pub fn width(&self) -> u64 {
        match self {
            Self::Pinhole(camera) => camera.width,
        }
    }

    pub fn focal_length_x(&self) -> f64 {
        match self {
            Self::Pinhole(camera) => camera.focal_length_x,
        }
    }

    pub fn focal_length_y(&self) -> f64 {
        match self {
            Self::Pinhole(camera) => camera.focal_length_y,
        }
    }

    pub fn principal_point_x(&self) -> f64 {
        match self {
            Self::Pinhole(camera) => camera.principal_point_x,
        }
    }

    pub fn principal_point_y(&self) -> f64 {
        match self {
            Self::Pinhole(camera) => camera.principal_point_y,
        }
    }
}

impl Decoder for Camera {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let [camera_id, model_id] = read_any::<[u32; 2]>(reader)?;
        let [width, height] = read_any::<[u64; 2]>(reader)?;
        let [focal_length_x, focal_length_y] = match model_id {
            0 => {
                let focal_length = read_any::<f64>(reader)?;
                [focal_length, focal_length]
            },
            1 => read_any::<[f64; 2]>(reader)?,
            _ => return Err(Error::UnknownCameraModelId(model_id)),
        };
        let [principal_point_x, principal_point_y] =
            read_any::<[f64; 2]>(reader)?;

        match model_id {
            0 | 1 => Ok(Self::Pinhole(PinholeCamera {
                camera_id,
                width,
                height,
                focal_length_x,
                focal_length_y,
                principal_point_x,
                principal_point_y,
            })),
            _ => Err(Error::UnknownCameraModelId(model_id)),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::Pinhole(Default::default())
    }
}
