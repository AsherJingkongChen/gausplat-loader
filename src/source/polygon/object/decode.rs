pub use super::*;

use std::io::{BufReader, Read};

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let reader = &mut BufReader::new(reader);
        let header = Header::decode(reader)?;
        let payload = Payload::decode_with(reader, &header)?;
        Ok(Self { header, payload })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn decode_on_example_triangle() {
        use super::*;

        let reader = &mut Cursor::new(
            &include_bytes!("../../../../examples/data/polygon/triangle.binary-le.ply")[..],
        );
        let object = Object::decode(reader).unwrap();

        let target = &vec![vec![
            vec![
                0x8f, 0xc3, 0xf5, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x8f, 0xc2, 0x75, 0xbe,
            ],
            vec![
                0x8f, 0xc2, 0xf5, 0xbd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00,
            ],
        ]];
        let output = object.get_elements().1;
        assert_eq!(output, target);

        let target = &target[0];
        let output = object.get_properties("vertex").unwrap().1;
        assert_eq!(output, target);

        let target = &[0.1200019046664238, 0.0, -0.23999999463558197];
        let output = object.get_property_as::<f32>("vertex", "x").unwrap();
        assert_eq!(output.1, target);
        let output = output.0.try_unwrap_scalar_ref().unwrap().size().unwrap();
        assert_eq!(output, size_of::<f32>());

        let target = &[-0.11999999731779099, 0.0, 1.1754943508222875e-38];
        let output = object.get_property_as::<f32>("vertex", "y").unwrap();
        assert_eq!(output.1, target);
        let output = output.0.try_unwrap_scalar_ref().unwrap().size().unwrap();
        assert_eq!(output, size_of::<f32>());

        let target = None;
        let output = object.get_property_as::<u64>("vertex", "x");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_property("vretex", "x");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_property("vertex", "z");
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_invalid_property_kind() {
        use super::*;

        let reader = &mut Cursor::new(
            &b"ply\nformat binary_little_endian 1.0\nelement point 1\nproperty flaot x\nend_header\n\0\0\0\0"[..],
        );
        let target = true;
        let output =
            matches!(Object::decode(reader).unwrap_err(), InvalidKind(k) if k == "flaot");
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_no_data() {
        use super::*;
        use std::io::ErrorKind::*;

        let reader = &mut Cursor::new(
            &b"ply\nformat binary_little_endian 1.0\nelement point 1\nproperty float x\nend_header\n"[..],
        );
        let target = true;
        let output = matches!(Object::decode(reader).unwrap_err(), Io(e) if e.kind() == UnexpectedEof);
        assert_eq!(output, target);
    }

    #[test]
    #[should_panic]
    fn decode_on_unimplemented_features_1() {
        use super::*;

        let reader = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
            )[..],
        );
        Object::decode(reader).unwrap_err();
    }

    #[test]
    #[should_panic]
    fn decode_on_unimplemented_features_2() {
        use super::*;

        let reader = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
            )[..],
        );
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
            )[..],
        );
        Object::decode(reader).unwrap_err();
    }

    // TODO:
    // - encode object tests
    // - encode object and access properties tests
    // - decode object tests
    // - decode two diff endian and compare tests
}
