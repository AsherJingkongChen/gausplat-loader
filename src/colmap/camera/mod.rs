pub mod cameras;
pub mod pinhole;

use crate::{
    error::*,
    function::{try_read_to_slice, Decoder},
};
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

impl Decoder for Camera {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let [id, model_id] = try_read_to_slice!(reader, u32, 2)?;
        let [width, height] = try_read_to_slice!(reader, u64, 2)?;

        match model_id {
            0 | 1 => {
                let [focal_length_x, focal_length_y] = match model_id {
                    0 => {
                        let [focal_length] =
                            try_read_to_slice!(reader, f64, 1)?;
                        [focal_length, focal_length]
                    },
                    1 => try_read_to_slice!(reader, f64, 2)?,
                    _ => unreachable!(),
                };
                let [principal_point_x, principal_point_y] =
                    try_read_to_slice!(reader, f64, 2)?;
                Ok(Self::Pinhole(PinholeCamera {
                    id,
                    width,
                    height,
                    focal_length_x,
                    focal_length_y,
                    principal_point_x,
                    principal_point_y,
                }))
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
            _ => Err(DecodeError::UnknownCameraModelId(model_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn camera_decode_io_error() {
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(&[]);
        let camera = Camera::decode(reader);
        let result = match camera {
            Err(DecodeError::Io(_)) => true,
            _ => false,
        };
        assert!(result, "{:#?}", camera);
    }

    #[test]
    fn camera_decode_pinhole_camera() {
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(
            b"\x01\x00\x00\x00\
            \x01\x00\x00\x00\
            \xa7\x07\x00\x00\x00\x00\x00\x00\
            \x42\x04\x00\x00\x00\x00\x00\x00\
            \xfe\x5d\xe3\x2f\x5a\x1e\x92\x40\
            \xfb\x66\xca\xf8\xa3\x32\x92\x40\
            \x00\x00\x00\x00\x00\x9c\x8e\x40\
            \x00\x00\x00\x00\x00\x08\x81\x40",
        );

        let camera = Camera::decode(reader);
        assert!(camera.is_ok(), "{:#?}", camera.unwrap_err());

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
