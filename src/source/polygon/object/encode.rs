pub use super::*;

use std::io::{BufWriter, Write};

impl Encoder for Object {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        let writer = &mut BufWriter::new(writer);

        self.header.encode(writer)?;
        assert!(
            !self.header.format.is_ascii(),
            "Unimplemented: ASCII format encoding"
        );

        let should_reverse_datum = !self.header.format.is_binary_native_endian();

        let elements = self.get_elements();
        elements.0.values().zip(elements.1.iter()).try_for_each(
            |(elem, elem_data)| {
                let prop_count = elem.len();
                let prop_sizes = elem.property_sizes().collect::<Result<Vec<_>, _>>()?;
                (0..elem.count)
                    .try_fold(vec![0; prop_count], |mut prop_offsets, elem_index| {
                        prop_offsets
                            .iter_mut()
                            .zip(prop_sizes.iter().zip(elem_data.iter()))
                            .try_for_each(|(offset, (size, data))| {
                                let start = *offset;
                                let end = start + size;
                                *offset = end;

                                let datum = data.get(start..end).ok_or_else(|| {
                                    OutOfBounds(
                                        end,
                                        elem.count * size,
                                        format!("element index {elem_index}"),
                                    )
                                })?;
                                let result = if should_reverse_datum {
                                    let datum = &mut datum.to_owned();
                                    datum.reverse();
                                    writer.write_all(datum)
                                } else {
                                    writer.write_all(datum)
                                };

                                #[cfg(test)]
                                result.unwrap();
                                #[cfg(not(test))]
                                result?;

                                Ok::<_, Self::Err>(())
                            })?;
                        Ok::<_, Self::Err>(prop_offsets)
                    })
                    .map(drop)
            },
        )?;

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::polygon::object", "Object::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn encode_on_default() {
        use super::*;

        let object = Object::default();

        let target = b"ply\nformat binary_little_endian 1.0\nend_header\n";
        let output = &mut vec![];
        object.encode(output).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_invalid_header() {
        use super::*;

        let object = Object {
            header: Header {
                version: "\u{4e00}\u{9eDe}\u{96f6}".into(),
                ..Default::default()
            },
            payload: Default::default(),
        };
        let output = &mut vec![];
        object.encode(output).unwrap_err();
    }

    #[test]
    fn encode_on_invalid_kind() {
        use super::PropertyKind::*;
        use super::*;
        let target = &b"\
            ply\nformat binary_little_endian 1.0\n\
            element point 1\n\
            property double x\n\
            end_header\n\
            \0\0\0\0\0\0\0\0\
        "[..];

        let mut object = Object::default();
        let (elements, data) = object.get_mut_elements();
        elements.insert(
            "point".into(),
            (
                1,
                "point",
                [("x".into(), (Scalar("duoble".into()), "x").into())]
                    .into_iter()
                    .collect(),
            )
                .into(),
        );
        data.push(vec![vec![0x00; 8]]);

        let output = &mut vec![];
        object.encode(output).unwrap_err();

        object.get_mut_property("point", "x").unwrap().0.kind = Scalar("double".into());
        let output = &mut vec![];
        object.encode(output).unwrap();
        assert_eq!(output, target);

        object
            .get_mut_property("point", "x")
            .unwrap()
            .1
            .pop()
            .unwrap();
        let output = &mut vec![];
        object.encode(output).unwrap_err();
    }

    #[test]
    fn encode_on_no_native_endian() {
        use super::*;

        let source_be = &include_bytes!(
            "../../../../examples/data/polygon/triangle.binary-be.ply"
        )[..];
        let source_le = &include_bytes!(
            "../../../../examples/data/polygon/triangle.binary-le.ply"
        )[..];

        let object = Object::decode(&mut Cursor::new(source_be)).unwrap();
        let target = &mut vec![];
        object.encode(target).unwrap();

        let mut object = Object::decode(&mut Cursor::new(source_le)).unwrap();
        object.header.format = Format::BinaryBigEndian;
        let output = &mut vec![];
        object.encode(output).unwrap();
        assert_eq!(output, target);

        let source_be = &include_bytes!(
            "../../../../examples/data/polygon/triangle.binary-be.ply"
        )[..];
        let source_le = &include_bytes!(
            "../../../../examples/data/polygon/triangle.binary-le.ply"
        )[..];

        let object = Object::decode(&mut Cursor::new(source_le)).unwrap();
        let target = &mut vec![];
        object.encode(target).unwrap();

        let mut object = Object::decode(&mut Cursor::new(source_be)).unwrap();
        object.header.format = Format::BinaryLittleEndian;
        let output = &mut vec![];
        object.encode(output).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    #[should_panic]
    fn encode_on_unimplemented_ascii() {
        use super::*;

        let mut object = Object::default();
        object.header.format = Format::Ascii;
        let output = &mut vec![];
        object.encode(output).unwrap_err();
    }

    #[test]
    fn encode_on_triangle_in_memory() {
        use super::PropertyKind::*;
        use super::*;

        let target = &b"\
            ply\nformat binary_little_endian 1.0\n\
            element vertex 1\n\
            property float x\nproperty float y\n\
            end_header\n\
            \0\0\x80\0\0\0\0\0\
        "[..];

        let mut object = Object::default();
        let (elements, data) = object.get_mut_elements();
        elements.insert(
            "vertex".into(),
            (
                1,
                "vertex",
                [
                    ("x".into(), (Scalar("float".into()), "x").into()),
                    ("y".into(), (Scalar("float".into()), "y").into()),
                ]
                .into_iter()
                .collect(),
            )
                .into(),
        );
        data.resize(1, Default::default());
        let data = object.get_mut_properties("vertex").unwrap().1;
        data.extend(vec![
            vec![0x00, 0x00, 0x80, 0x00],
            vec![0x00, 0x00, 0x00, 0x00],
        ]);

        let output = &mut vec![];
        object.encode(output).unwrap();
        assert_eq!(output, target);

        object
            .get_mut_property("vertex", "x")
            .unwrap()
            .1
            .pop()
            .unwrap();
        let target = None;
        let output = object.get_mut_property_as::<f32>("vertex", "x");
        assert_eq!(output, target);
        let target = None;
        let output = object.get_mut_property_as::<f32>("vertex", "z");
        assert_eq!(output, target);

        object
            .get_mut_properties("vertex")
            .unwrap()
            .1
            .pop()
            .unwrap();
        let target = None;
        let output = object.get_mut_property("vertex", "y");
        assert_eq!(output, target);
        let target = None;
        let output = object.get_property("vertex", "y");
        assert_eq!(output, target);

        object
            .get_mut_properties("vertex")
            .unwrap()
            .0
            .pop()
            .unwrap();
        let target = None;
        let output = object.get_mut_property("vertex", "y");
        assert_eq!(output, target);
        let target = None;
        let output = object.get_mut_property_as::<f32>("vertex", "y");
        assert_eq!(output, target);

        object.get_mut_elements().1.pop().unwrap();
        let target = None;
        let output = object.get_mut_properties("vertex");
        assert_eq!(output, target);
        let target = None;
        let output = object.get_element("vertex");
        assert_eq!(output, target);

        object.get_mut_elements().0.pop().unwrap();
        let target = None;
        let output = object.get_mut_element("vertex");
        assert_eq!(output, target);
        let target = None;
        let output = object.get_element("vertex");
        assert_eq!(output, target);

        let target = None;
        let output = object.get_mut_property("vertex", "x");
        assert_eq!(output, target);
    }
}
