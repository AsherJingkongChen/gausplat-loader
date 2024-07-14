pub mod cameras;
pub mod pinhole;

pub use crate::function::Decoder;
use crate::{error::*, function::read_to_slice};
pub use cameras::*;
pub use pinhole::*;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub enum Camera {
    Pinhole(PinholeCamera),
    SimpleRadial,
    Radial,
    OpenCV,
    OpenCVFisheye,
    FullOpenCV,
    FOV,
    SimpleRadialFisheye,
    RadialFisheye,
    ThinPrismFisheye,
}

impl Camera {
    pub fn camera_id(&self) -> u32 {
        match self {
            Self::Pinhole(camera) => camera.camera_id,
            _ => unimplemented!(),
        }
    }

    pub fn height(&self) -> u64 {
        match self {
            Self::Pinhole(camera) => camera.height,
            _ => unimplemented!(),
        }
    }

    pub fn width(&self) -> u64 {
        match self {
            Self::Pinhole(camera) => camera.width,
            _ => unimplemented!(),
        }
    }
}

impl Decoder for Camera {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let [camera_id, model_id] = read_to_slice!(reader, u32, 2)?;
        let [width, height] = read_to_slice!(reader, u64, 2)?;

        match model_id {
            0..=1 => {
                let [focal_length_x, focal_length_y] = match model_id {
                    0 => {
                        let [focal_length] = read_to_slice!(reader, f64, 1)?;
                        [focal_length, focal_length]
                    },
                    1 => read_to_slice!(reader, f64, 2)?,
                    _ => unreachable!(),
                };
                let [principal_point_x, principal_point_y] =
                    read_to_slice!(reader, f64, 2)?;
                Ok(Self::Pinhole(PinholeCamera {
                    camera_id,
                    width,
                    height,
                    focal_length_x,
                    focal_length_y,
                    principal_point_x,
                    principal_point_y,
                }))
            },
            2..=10 => {
                Err(Error::UnsupportedCameraModel(match model_id {
                    2 => Self::SimpleRadial,
                    3 => Self::Radial,
                    4 => Self::OpenCV,
                    5 => Self::OpenCVFisheye,
                    6 => Self::FullOpenCV,
                    7 => Self::FOV,
                    8 => Self::SimpleRadialFisheye,
                    9 => Self::RadialFisheye,
                    10 => Self::ThinPrismFisheye,
                    _ => unreachable!(),
                }))
            },
            _ => Err(Error::UnknownCameraModelId(model_id)),
        }
    }
}
