pub use super::Camera;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::{read_any, write_any};
use std::io::{BufReader, BufWriter, Read, Write};

pub type Cameras = std::collections::HashMap<u32, Camera>;

impl Decoder for Cameras {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let reader = &mut BufReader::new(reader);
        let camera_count = read_any::<u64>(reader)? as usize;

        let cameras = (0..camera_count)
            .map(|_| {
                let camera = Camera::decode(reader)?;
                Ok((camera.camera_id(), camera))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::camera", "Cameras::decode");

        cameras
    }
}

impl Encoder for Cameras {
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error> {
        let writer = &mut BufWriter::new(writer);

        write_any(writer, &(self.len() as u64))?;
        for (camera_id, camera) in self.iter() {
            write_any(writer, camera_id)?;
            camera.encode(writer)?;
        }

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::camera", "Cameras::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::super::*;

        let source = include_bytes!("../../../../examples/data/cameras.bin");
        let mut reader = std::io::Cursor::new(source);

        let target_count = 2;
        let targets = [
            (
                1,
                Camera::Pinhole(PinholeCamera {
                    camera_id: 1,
                    width: 1959,
                    height: 1090,
                    focal_length_x: 1159.5880733038061,
                    focal_length_y: 1164.6601287484507,
                    principal_point_x: 979.5,
                    principal_point_y: 545.0,
                }),
            ),
            (
                2,
                Camera::Pinhole(PinholeCamera {
                    camera_id: 2,
                    width: 1957,
                    height: 1091,
                    focal_length_x: 1163.2547280302354,
                    focal_length_y: 1163.2547280302354,
                    principal_point_x: 978.5,
                    principal_point_y: 545.5,
                }),
            ),
        ]
        .into_iter()
        .collect();
        let outputs = Cameras::decode(&mut reader).unwrap();
        assert_eq!(outputs.len(), target_count);
        assert_eq!(outputs, targets);
    }

    #[test]
    fn decode_on_zero_bytes() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[]);

        Cameras::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn decode_on_zero_entries() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let outputs = Cameras::decode(&mut reader).unwrap();
        assert!(outputs.is_empty());
    }
}
