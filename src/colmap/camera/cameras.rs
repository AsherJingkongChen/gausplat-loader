use super::Camera;
pub use crate::function::Decoder;
use crate::{error::*, function::read_to_slice};
use std::collections::HashMap;
use std::io;

pub type Cameras = HashMap<u32, Camera>;

impl Decoder for Cameras {
    fn decode<R: io::Read>(
        reader: &mut R
    ) -> Result<Self, DecodeError> {
        let camera_count = read_to_slice!(reader, u64, 1)?[0] as usize;
        let mut cameras = Self::with_capacity(camera_count);

        for _ in 0..camera_count {
            let camera = Camera::decode(reader)?;
            match &camera {
                Camera::Pinhole(pinhole) => {
                    cameras.insert(pinhole.camera_id, camera);
                },
                _ => return Err(DecodeError::UnsupportedCameraModel(camera)),
            }
        }

        Ok(cameras)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cameras_decode_zero_bytes() {
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(&[]);

        let cameras = Cameras::decode(reader);
        assert!(cameras.is_err(), "{:#?}", cameras.unwrap());
    }

    #[test]
    fn cameras_decode_zero_entries() {
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let cameras = Cameras::decode(reader);
        assert!(cameras.is_ok(), "{:#?}", cameras.unwrap_err());

        let cameras = cameras.unwrap();
        assert!(cameras.is_empty());
    }

    #[test]
    fn cameras_decode() {
        use super::super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(&[
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0xa7, 0x07, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x42, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe,
            0x5d, 0xe3, 0x2f, 0x5a, 0x1e, 0x92, 0x40, 0xfb, 0x66, 0xca, 0xf8,
            0xa3, 0x32, 0x92, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x8e,
            0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x81, 0x40, 0x02, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0x07, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x43, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xf1, 0xbc, 0x6c, 0xd7, 0x04, 0x2d, 0x92, 0x40, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x94, 0x8e, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c,
            0x81, 0x40,
        ]);

        let cameras = Cameras::decode(reader);
        assert!(cameras.is_ok(), "{:#?}", cameras.unwrap_err());

        let cameras = cameras.unwrap();
        assert_eq!(cameras.len(), 2);
        assert_eq!(
            cameras.get(&1),
            Some(&Camera::Pinhole(PinholeCamera {
                camera_id: 1,
                width: 1959,
                height: 1090,
                focal_length_x: 1159.5880733038061,
                focal_length_y: 1164.6601287484507,
                principal_point_x: 979.5,
                principal_point_y: 545.0,
            }))
        );
        assert_eq!(
            cameras.get(&2),
            Some(&Camera::Pinhole(PinholeCamera {
                camera_id: 2,
                width: 1957,
                height: 1091,
                focal_length_x: 1163.2547280302354,
                focal_length_y: 1163.2547280302354,
                principal_point_x: 978.5,
                principal_point_y: 545.5,
            }))
        );
    }
}
