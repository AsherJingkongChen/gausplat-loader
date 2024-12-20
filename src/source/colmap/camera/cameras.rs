//! A collection of cameras.
//!
//! It is generally stored in the `cameras.bin` file.

pub use super::*;

/// A map of [`Camera::camera_id`] to [`Camera`].
pub type Cameras = crate::collection::IndexMap<u32, Camera>;

impl Decoder for Cameras {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let reader = &mut BufReader::new(reader);

        let camera_count = reader.read_u64::<LE>()?;
        let cameras = (0..camera_count)
            .map(|_| {
                let camera = Camera::decode(reader)?;
                Ok((camera.camera_id, camera))
            })
            .collect();

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::colmap::camera", "Cameras::decode");

        cameras
    }
}

impl Encoder for Cameras {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        let writer = &mut BufWriter::new(writer);

        writer.write_u64::<LE>(self.len() as u64)?;
        self.values().try_for_each(|camera| camera.encode(writer))?;

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::colmap::camera", "Cameras::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::super::*;
        use std::io::Cursor;

        let source =
            &include_bytes!("../../../../examples/data/colmap/0/cameras.bin")[..];

        let targets = [
            (
                1,
                Camera {
                    camera_id: 1,
                    width: 1959,
                    height: 1090,
                    principal_point_x: 979.5,
                    principal_point_y: 545.0,
                    variant: CameraVariant::Pinhole {
                        focal_length_x: 1159.5880733038061,
                        focal_length_y: 1164.6601287484507,
                    },
                },
            ),
            (
                2,
                Camera {
                    camera_id: 2,
                    width: 1957,
                    height: 1091,
                    principal_point_x: 978.5,
                    principal_point_y: 545.5,
                    variant: CameraVariant::SimplePinhole {
                        focal_length: 1163.2547280302354,
                    },
                },
            ),
        ]
        .into_iter()
        .collect::<Cameras>();

        (0..source.len()).for_each(|i| {
            let mut reader = Cursor::new(&source[..i]);
            Cameras::decode(&mut reader).unwrap_err();
        });
        let mut reader = Cursor::new(source);
        let output = Cameras::decode(&mut reader).unwrap();
        assert_eq!(output, targets);

        let target = false;
        let camera = targets.get(&1).unwrap();
        let output = camera.focal_length_x() == camera.focal_length_y();
        assert_eq!(output, target);

        let target = true;
        let camera = targets.get(&2).unwrap();
        let output = camera.focal_length_x() == camera.focal_length_y();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_unknown_camera_model_id() {
        use super::*;

        let source =
            &include_bytes!("../../../../examples/data/colmap/2/cameras.bin")[..];
        let mut reader = std::io::Cursor::new(source);

        let target = -1_i32 as u32;
        let output = matches!(
            Cameras::decode(&mut reader).unwrap_err(),
            Error::InvalidCameraModelId(id) if id == target,
        );
        let target = true;
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_zero_bytes() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&b""[..]);
        Cameras::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn decode_on_zero_entry() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0][..]);

        let target = true;
        let output = Cameras::decode(&mut reader).unwrap().is_empty();
        assert_eq!(output, target);

        let mut reader = Cursor::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0][..]);
        Cameras::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn encode() {
        use super::super::*;
        use std::io::Cursor;

        let source = [
            (
                1,
                Camera {
                    camera_id: 1,
                    width: 1959,
                    height: 1090,
                    principal_point_x: 979.5,
                    principal_point_y: 545.0,
                    variant: CameraVariant::Pinhole {
                        focal_length_x: 1159.5880733038061,
                        focal_length_y: 1164.6601287484507,
                    },
                },
            ),
            (
                2,
                Camera {
                    camera_id: 2,
                    width: 1957,
                    height: 1091,
                    principal_point_x: 978.5,
                    principal_point_y: 545.5,
                    variant: CameraVariant::SimplePinhole {
                        focal_length: 1163.2547280302354,
                    },
                },
            ),
        ]
        .into_iter()
        .collect::<Cameras>();

        let target =
            &include_bytes!("../../../../examples/data/colmap/0/cameras.bin")[..];
        let mut writer = Cursor::new(vec![]);
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_zero_entry() {
        use super::*;
        use std::io::Cursor;

        let source = Cameras::default();

        let target = &[0, 0, 0, 0, 0, 0, 0, 0][..];
        let mut writer = Cursor::new(vec![]);
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }
}
