pub use super::*;

use crate::function::{
    is_space, read_byte_after, read_bytes_before, read_bytes_before_newline,
    read_bytes_const, read_bytes_until_many_const, read_newline, string_from_vec_ascii,
};
use std::io::Read;

impl Decoder for Header {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        read_bytes_until_many_const(reader, b"ply")?;
        read_newline(reader)?;

        if &read_bytes_const(reader)? != b"format " {
            return Err(MissingSymbol("format ".into()));
        }

        let format = match &read_bytes_const(reader)? {
            b"b" => {
                if &read_bytes_const(reader)? != b"inary_" {
                    return Err(MissingSymbol(format!(
                        "{} or {}",
                        BinaryLittleEndian, BinaryBigEndian
                    )));
                }
                match &read_bytes_const(reader)? {
                    b"l" => {
                        if &read_bytes_const(reader)? != b"ittle_endian " {
                            return Err(MissingSymbol(BinaryLittleEndian.to_string()));
                        }
                        BinaryLittleEndian
                    },
                    b"b" => {
                        if &read_bytes_const(reader)? != b"ig_endian " {
                            return Err(MissingSymbol(BinaryBigEndian.to_string()));
                        }
                        BinaryBigEndian
                    },
                    _ => {
                        return Err(MissingSymbol(format!(
                            "{} or {}",
                            BinaryLittleEndian, BinaryBigEndian
                        )))
                    },
                }
            },
            b"a" => {
                if &read_bytes_const(reader)? != b"scii " {
                    return Err(MissingSymbol(Ascii.to_string()));
                }
                Ascii
            },
            _ => return Err(MissingSymbol("ascii or binary".into())),
        };

        let version = string_from_vec_ascii(read_bytes_before_newline(reader, 4)?)?;

        let mut elements = Elements::default();

        loop {
            match &read_bytes_const(reader)? {
                b"end_head" => {
                    if &read_bytes_const(reader)? != b"er" {
                        return Err(MissingSymbol("end_header".into()));
                    }
                    read_newline(reader)?;
                    break;
                },
                b"property" => {
                    if &read_bytes_const(reader)? != b" " {
                        return Err(MissingSymbol(" ".into()));
                    }

                    let properties = &mut elements
                        .last_mut()
                        .ok_or_else(|| MissingSymbol("element".into()))?
                        .1
                        .properties;

                    let mut value = vec![read_byte_after(reader, is_space)?];
                    value.extend(read_bytes_before(reader, is_space, 8)?);
                    let value = string_from_vec_ascii(value)?;

                    let kind = if value != "list" {
                        Scalar(value.into())
                    } else {
                        let mut kind = vec![read_byte_after(reader, is_space)?];
                        kind.extend(read_bytes_before(reader, is_space, 8)?);
                        let count = string_from_vec_ascii(kind)?;

                        let mut kind = vec![read_byte_after(reader, is_space)?];
                        kind.extend(read_bytes_before(reader, is_space, 8)?);
                        let value = string_from_vec_ascii(kind)?;

                        List((count.into(), value.into()).into())
                    };

                    let name =
                        string_from_vec_ascii(read_bytes_before_newline(reader, 16)?)?;

                    properties.insert(name.to_owned(), Property { kind, name });
                },
                b"element " => {
                    let properties = Default::default();

                    let mut name = vec![read_byte_after(reader, is_space)?];
                    name.extend(read_bytes_before(reader, is_space, 16)?);
                    let name = string_from_vec_ascii(name)?;

                    let count =
                        string_from_vec_ascii(read_bytes_before_newline(reader, 8)?)?
                            .parse::<usize>()?;

                    elements.insert(
                        name.to_owned(),
                        Element {
                            count,
                            name,
                            properties,
                        },
                    );
                },
                b"comment " | b"obj_info" => {
                    drop(read_bytes_before_newline(reader, 64)?);
                },
                _ => {
                    return Err(MissingSymbol(
                        "comment, element, end_header, obj_info, or property".into(),
                    ))
                },
            }
        }

        Ok(Header {
            format,
            elements,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn decode_on_example_another_cube() {
        use super::*;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
            )[..],
        );

        let header = Header::decode(source).unwrap();

        let target = &Ascii;
        let output = &header.format;
        assert_eq!(output, target);

        let target = "1.0";
        let output = &header.version;
        assert_eq!(output, target);

        let target = &vec!["vertex", "face", "edge"];
        let output = &header.elements.keys().collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![8, 7, 5];
        let output = &header
            .elements
            .values()
            .map(|e| e.count)
            .collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![
            (Scalar("float".to_string().into()), "x".into()).into(),
            (Scalar("float".to_string().into()), "y".into()).into(),
            (Scalar("float".to_string().into()), "z".into()).into(),
            (Scalar("uchar".to_string().into()), "red".into()).into(),
            (Scalar("uchar".to_string().into()), "green".into()).into(),
            (Scalar("uchar".to_string().into()), "blue".into()).into(),
        ];
        let output = &header.elements["vertex"]
            .properties
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![(
            List(("uchar".to_string().into(), "int".to_string().into()).into()),
            "vertex_index".to_string(),
        )
            .into()];
        let output = &header.elements["face"]
            .properties
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![
            (Scalar("int".to_string().into()), "vertex1".into()).into(),
            (Scalar("int".to_string().into()), "vertex2".into()).into(),
            (Scalar("uchar".to_string().into()), "red".into()).into(),
            (Scalar("uchar".to_string().into()), "green".into()).into(),
            (Scalar("uchar".to_string().into()), "blue".into()).into(),
        ];
        let output = &header.elements["edge"]
            .properties
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(output, target);
    }

    /// This test **ensures** the decoding results are all `Err`.
    #[test]
    fn decode_on_example_err() {
        use super::*;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/duplicate-format.ascii.ply"
            )[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/misplaced-property.ascii.ply"
            )[..],
        );
        Header::decode(source).unwrap_err();
    }
    /// This test **ensures** the decoding results are all `Ok`.
    #[test]
    fn decode_on_example_ok() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.zeros.binary-le.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!("../../../../examples/data/polygon/empty-element.ascii.ply")
                [..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/empty-element.binary-le.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!("../../../../examples/data/polygon/empty-head.ascii.ply")[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/supported-data-types-common.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/supported-data-types-special.ply"
            )[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!("../../../../examples/data/polygon/triangle.binary-le.ply")[..],
        );
        Header::decode(source).unwrap();

        let source = &mut Cursor::new(
            &include_bytes!("../../../../examples/data/polygon/valid-keyword.ascii.ply")
                [..],
        );
        Header::decode(source).unwrap();
    }

    #[test]
    fn decode_on_no_signature() {
        use super::*;

        let source = &mut Cursor::new(&b"plr"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_format_keyword() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\n"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_format() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformta "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat e"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binarg_"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_B"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_little"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_little_endiam "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_big"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_big_endien "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat acsii "[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_version() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_big_endian 1.0"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat binary_little_endian 1.0"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&"ply\nformat ascii \u{b9}.\u{ba}\n"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.\xff\n"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_keyword() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nunknown"[..]);
        Header::decode(source).unwrap_err();

        let source =
            &mut Cursor::new(&b"ply\nformat ascii 1.0\nunknown \nend_header\n"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_end_header() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nend_head\n"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nend_headre"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nend_header"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_property() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nproperty"[..]);
        Header::decode(source).unwrap_err();

        let source =
            &mut Cursor::new(&b"ply\nformat ascii 1.0\nproperty\nend_header\n"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_property_kind() {
        use super::*;

        let source = &mut Cursor::new(
            &include_bytes!(
                "../../../../examples/data/polygon/misplaced-property.ascii.ply"
            )[..],
        );
        Header::decode(source).unwrap_err();

        let source =
            &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement vertex 0\nproperty  "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty float"[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &"ply\nformat ascii 1.0\nelement vertex 0\nproperty \u{ae} "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty \xff "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list uchar"[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &"ply\nformat ascii 1.0\nelement vertex 0\nproperty list \u{ae} "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list \xff "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list uchar "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list uchar int"[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &"ply\nformat ascii 1.0\nelement vertex 0\nproperty list uchar \u{ae} "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty list uchar \xff "[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty float x"[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &"ply\nformat ascii 1.0\nelement vertex 0\nproperty float \u{ae}\n"[..],
        );
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(
            &b"ply\nformat ascii 1.0\nelement vertex 0\nproperty float \xff\n"[..],
        );
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_element() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement vertex"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&"ply\nformat ascii 1.0\nelement \u{ae} "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement \xff "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement vertex 1"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement vertex 1 \n"[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&"ply\nformat ascii 1.0\nelement vertex g\n"[..]);
        Header::decode(source).unwrap_err();

        let source =
            &mut Cursor::new(&"ply\nformat ascii 1.0\nelement vertex \u{ae}\n"[..]);
        Header::decode(source).unwrap_err();

        let source =
            &mut Cursor::new(&b"ply\nformat ascii 1.0\nelement vertex \xff\n"[..]);
        Header::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_no_comment() {
        use super::*;

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\ncomment  "[..]);
        Header::decode(source).unwrap_err();

        let source = &mut Cursor::new(&b"ply\nformat ascii 1.0\nobj_info  "[..]);
        Header::decode(source).unwrap_err();
    }
}
