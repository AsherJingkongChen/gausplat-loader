use super::Camera;
use crate::{
    error::*,
    function::{try_read_to_slice, Decoder},
};
use std::collections::HashMap;
use std::io;

pub type Cameras = HashMap<u32, Camera>;

impl Decoder for Cameras {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let [camera_count] = try_read_to_slice!(reader, u64, 1)?;
        let mut cameras = HashMap::with_capacity(camera_count as usize);

        for _ in 0..camera_count {
            let camera = Camera::decode(reader)?;
            match &camera {
                Camera::Pinhole(pinhole) => {
                    cameras.insert(pinhole.id, camera);
                },
                _ => {
                    return Err(DecodeError::UnsupportedCameraModelType(camera))
                },
            }
        }

        Ok(cameras)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cameras_decode() {
        use super::super::*;
        use super::*;
        use std::io::Cursor;

        let reader = &mut Cursor::new(
            b"\x02\x00\x00\x00\x00\x00\x00\x00\
            \x01\x00\x00\x00\
            \x01\x00\x00\x00\
            \xa7\x07\x00\x00\x00\x00\x00\x00\
            \x42\x04\x00\x00\x00\x00\x00\x00\
            \xfe\x5d\xe3\x2f\x5a\x1e\x92\x40\
            \xfb\x66\xca\xf8\xa3\x32\x92\x40\
            \x00\x00\x00\x00\x00\x9c\x8e\x40\
            \x00\x00\x00\x00\x00\x08\x81\x40\
            \x02\x00\x00\x00\
            \x00\x00\x00\x00\
            \xa5\x07\x00\x00\x00\x00\x00\x00\
            \x43\x04\x00\x00\x00\x00\x00\x00\
            \xf1\xbc\x6c\xd7\x04\x2d\x92\x40\
            \x00\x00\x00\x00\x00\x94\x8e\x40\
            \x00\x00\x00\x00\x00\x0c\x81\x40",
        );

        let cameras = Cameras::decode(reader);
        assert!(cameras.is_ok(), "{:#?}", cameras.unwrap_err());

        let cameras = cameras.unwrap();
        assert_eq!(
            cameras.get(&1),
            Some(&Camera::Pinhole(PinholeCamera {
                id: 1,
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
                id: 2,
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
