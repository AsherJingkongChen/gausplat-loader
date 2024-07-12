pub mod pinhole;

use crate::{error::*, function::Decoder};
use bytemuck::try_from_bytes;
pub use pinhole::*;
use std::{io, mem::size_of};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Camera {
    SimplePinhole,
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

impl Decoder for Camera {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let [id, model_id] = {
            type T = u32;
            const N: usize = 2;
            let mut bytes = [0; N * size_of::<T>()];
            reader.read(&mut bytes).map_err(DecodeError::Io)?;
            *try_from_bytes::<[T; N]>(&bytes).map_err(DecodeError::CastError)?
        };

        match model_id {
            0 => Ok(Self::SimplePinhole),
            1 => {
                let [width, height] = {
                    type T = u64;
                    const N: usize = 2;
                    let mut bytes = [0; N * size_of::<T>()];
                    reader.read(&mut bytes).map_err(DecodeError::Io)?;
                    *try_from_bytes::<[T; N]>(&bytes)
                        .map_err(DecodeError::CastError)?
                };
                let [focal_length_x, focal_length_y, principal_point_x, principal_point_y] = {
                    type T = f64;
                    const N: usize = 4;
                    let mut bytes = [0; N * size_of::<T>()];
                    reader.read(&mut bytes).map_err(DecodeError::Io)?;
                    *try_from_bytes::<[T; N]>(&bytes)
                        .map_err(DecodeError::CastError)?
                };
                let camera = PinholeCamera {
                    id,
                    width,
                    height,
                    focal_length_x,
                    focal_length_y,
                    principal_point_x,
                    principal_point_y,
                };
                Ok(Self::Pinhole(camera))
            },
            2 => Ok(Self::SimpleRadial),
            3 => Ok(Self::Radial),
            4 => Ok(Self::OpenCV),
            5 => Ok(Self::OpenCVFisheye),
            6 => Ok(Self::FullOpenCV),
            7 => Ok(Self::FOV),
            8 => Ok(Self::SimpleRadialFisheye),
            9 => Ok(Self::RadialFisheye),
            10 => Ok(Self::ThinPrismFisheye),
            _ => Err(DecodeError::InvalidCameraModelId(model_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn camera_decode_pinhole_camera() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(
            b"\x01\x00\x00\x00\
            \x01\x00\x00\x00\
            \xa7\x07\x00\x00\x00\x00\x00\x00\
            \x42\x04\x00\x00\x00\x00\x00\x00\
            \xfe\x5d\xe3\x2f\x5a\x1e\x92\x40\
            \xfb\x66\xca\xf8\xa3\x32\x92\x40\
            \x00\x00\x00\x00\x00\x9c\x8e\x40\
            \x00\x00\x00\x00\x00\x08\x81\x40",
        );

        let camera = Camera::decode(&mut reader);
        assert!(camera.is_ok(), "{:?}", camera.unwrap_err());

        let camera = camera.unwrap();
        assert_eq!(
            camera,
            Camera::Pinhole(PinholeCamera {
                id: 1,
                width: 1959,
                height: 1090,
                focal_length_x: 1159.5880733038061,
                focal_length_y: 1164.6601287484507,
                principal_point_x: 979.5,
                principal_point_y: 545.0,
            })
        );
    }
}
