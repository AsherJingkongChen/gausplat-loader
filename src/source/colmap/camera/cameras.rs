pub use super::Camera;
pub use crate::function::Decoder;

use crate::{error::Error, function::read_slice};
use std::io::{BufReader, Read};

pub type Cameras = std::collections::HashMap<u32, Camera>;

impl Decoder for Cameras {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let reader = &mut BufReader::new(reader);
        let camera_count = read_slice::<u64, 1>(reader)?[0] as usize;

        let cameras = (0..camera_count)
            .map(|_| {
                let camera = Camera::decode(reader)?;
                Ok((camera.camera_id(), camera))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat_loader::source", "colmap::Cameras::decode");

        cameras
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_zero_bytes() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[]);

        let cameras = Cameras::decode(&mut reader);
        assert!(cameras.is_err(), "{:#?}", cameras.unwrap());
    }

    #[test]
    fn decode_zero_entries() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let cameras = Cameras::decode(&mut reader).unwrap();
        assert!(cameras.is_empty());
    }

    #[test]
    fn decode() {
        use super::super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[
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

        let cameras = Cameras::decode(&mut reader).unwrap();
        assert_eq!(cameras.len(), 2);
        assert_eq!(
            cameras.get(&1),
            Some(&Camera::Pinhole(PinholeCamera {
                camera_id: 1,
                width: 1959,
                height: 1090,
                focal_length_x: 1159.5880733038061,
                focal_length_y: 1164.6601287484507,
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
            }))
        );
    }
}
