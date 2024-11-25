pub use super::*;

use std::io::{BufReader, Read};

impl Decoder for Object {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let reader = &mut BufReader::new(reader);
        let header = Header::decode(reader)?;
        let payload = Payload::decode_with(reader, &header)?;

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::polygon::object", "Object::decode");

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
        let mut object = Object::decode(reader).unwrap();

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

        let target = None;
        let output = object.get_properties("vretex");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_element("vretex");
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
        let output = object.get_property("vretex", "x");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_property("vertex", "z");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_property_as::<f32>("vertex", "z");
        assert_eq!(output, target);

        let target = None;
        object
            .get_mut_property("vertex", "x")
            .unwrap()
            .1
            .pop()
            .unwrap();
        let output = object.get_property_as::<f32>("vertex", "x");
        assert_eq!(output, target);

        let target = &[2.0_f32, -0.3];
        *object.get_mut_property("vertex", "x").unwrap().1 =
            try_cast_slice(target).unwrap().to_owned();
        let output = object.get_property_as::<f32>("vertex", "x").unwrap();
        assert_eq!(output.1, target);

        let target = &[-1.23_f32, 0.0, 0.1];
        object
            .get_mut_property_as("vertex", "y")
            .unwrap()
            .1
            .copy_from_slice(target);
        let output = object.get_property_as::<f32>("vertex", "y").unwrap();
        assert_eq!(output.1, target);
    }

    #[test]
    fn decode_on_invalid_header() {
        use super::*;

        let source = &mut Cursor::new(
            &b"ply\nformat binary_little_endian 1.0\n\
            element point 1\n\
            property float x\n\
            end_header\
            \0\0\0\0"[..],
        );
        Object::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_property_kind() {
        use super::*;

        let source = &mut Cursor::new(
            &b"ply\nformat binary_little_endian 1.0\n\
            element point 1\n\
            property flaot x\n\
            end_header\n\
            \0\0\0\0"[..],
        );
        let target = true;
        let output =
            matches!(Object::decode(source).unwrap_err(), InvalidKind(k) if k == "flaot");
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_no_data() {
        use super::*;
        use std::io::ErrorKind::*;

        let source = &mut Cursor::new(
            &b"ply\nformat binary_little_endian 1.0\n\
            element point 1\n\
            property float x\n\
            end_header\n"[..],
        );
        let target = true;
        let output = matches!(Object::decode(source).unwrap_err(), Io(e) if e.kind() == UnexpectedEof);
        assert_eq!(output, target);
    }

    #[test]
    #[should_panic]
    fn decode_on_no_implemented_ascii() {
        use super::*;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
            )[..],
        );
        Object::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_non_scalar_payload() {
        use super::*;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
            )[..],
        );
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
            )[..],
        );
        Object::decode(source).unwrap_err();
    }
}
