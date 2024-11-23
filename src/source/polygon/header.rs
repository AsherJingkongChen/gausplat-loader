pub use super::*;
pub use indexmap::IndexMap;

use crate::function::{
    is_space, read_byte_after, read_bytes_before, read_bytes_before_newline,
    read_bytes_const, read_bytes_until_many_const, read_newline, string_from_vec_ascii,
};
use derive_more::derive::From;
use std::{fmt, io::Read};
use Error::*;
use Format::*;
use PropertyKind::*;

#[derive(Clone, Debug, Default, Eq, From, PartialEq)]
pub struct Header {
    pub elements: IndexMap<String, Element>,
    pub format: Format,
    pub version: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, From, Hash, PartialEq)]
pub enum Format {
    #[default]
    BinaryLittleEndian,
    Ascii,
    BinaryBigEndian,
}

#[derive(Clone, Debug, Default, Eq, From, PartialEq)]
pub struct Element {
    pub properties: Properties,
    pub name: String,
    pub size: usize,
}

#[derive(Clone, Debug, Default, Eq, From, Hash, PartialEq)]
pub struct Property {
    pub kind: PropertyKind,
    pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, From, PartialEq)]
pub enum PropertyKind {
    List { count: String, value: String },
    Scalar(String),
}

pub type Properties = IndexMap<String, Property>;

impl fmt::Display for Format {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            BinaryLittleEndian => write!(f, "binary_little_endian"),
            Ascii => write!(f, "ascii"),
            BinaryBigEndian => write!(f, "binary_big_endian"),
        }
    }
}

impl Default for PropertyKind {
    fn default() -> Self {
        Scalar(Default::default())
    }
}

impl Decoder for Header {
    type Err = Error;

    // TODO: Spotting all branch failures causes for the function `decode`.
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        // Fail: No "ply"
        read_bytes_until_many_const(reader, b"ply")?;
        // Fail: No newline
        read_newline(reader)?;

        // Fail: No 7 bytes or no "format "
        if &read_bytes_const(reader)? != b"format " {
            Err(MissingSymbol("format ".into()))?;
        }

        // Fail: No 1 byte or no "a" or "b"
        let format = match &read_bytes_const(reader)? {
            b"b" => {
                // Fail: No 6 bytes or no "inary_"
                if &read_bytes_const(reader)? != b"inary_" {
                    Err(MissingSymbol(format!(
                        "{} or {}",
                        BinaryLittleEndian, BinaryBigEndian
                    )))?;
                }
                // Fail: No 1 byte or no "l" or "b"
                match &read_bytes_const(reader)? {
                    b"l" => {
                        // Fail: No 13 bytes or no "ittle_endian "
                        if &read_bytes_const(reader)? != b"ittle_endian " {
                            Err(MissingSymbol(BinaryLittleEndian.to_string()))?;
                        }
                        BinaryLittleEndian
                    },
                    b"b" => {
                        // Fail: No 10 bytes or no "ig_endian "
                        if &read_bytes_const(reader)? != b"ig_endian " {
                            Err(MissingSymbol(BinaryBigEndian.to_string()))?;
                        }
                        BinaryBigEndian
                    },
                    _ => Err(MissingSymbol(format!(
                        "{} or {}",
                        BinaryLittleEndian, BinaryBigEndian
                    )))?,
                }
            },
            b"a" => {
                // Fail: No 5 bytes or no "scii "
                if &read_bytes_const(reader)? != b"scii " {
                    Err(MissingSymbol(Ascii.to_string()))?;
                }
                Ascii
            },
            _ => Err(MissingSymbol("ascii or binary".into()))?,
        };

        // Fail: No newline or no utf8 or ascii
        let version = string_from_vec_ascii(read_bytes_before_newline(reader, 4)?)?;

        let mut elements = IndexMap::<String, Element>::new();
        // Fail: No 8 bytes or no match

        loop {
            match &read_bytes_const(reader)? {
                b"end_head" => {
                    // Fail: No 2 bytes or no "er"
                    if &read_bytes_const(reader)? != b"er" {
                        Err(MissingSymbol("end_header".into()))?;
                    }
                    // Fail: No newline
                    read_newline(reader)?;
                    break;
                },
                b"property" => {
                    // Fail: No 1 byte or no " "
                    if &read_bytes_const(reader)? != b" " {
                        Err(MissingSymbol(" ".into()))?;
                    }

                    // Fail: No element (misplaced property)
                    let properties = &mut elements
                        .last_mut()
                        .ok_or_else(|| MissingSymbol("element".into()))?
                        .1
                        .properties;

                    // Fail: Only space
                    let mut kind = vec![read_byte_after(reader, is_space)?];
                    // Fail: No space
                    kind.extend(read_bytes_before(reader, is_space, 8)?);

                    let kind = if kind != b"list" {
                        // Fail: No utf8 or ascii
                        Scalar(string_from_vec_ascii(kind)?)
                    } else {
                        // Fail: Only space
                        let mut kind = vec![read_byte_after(reader, is_space)?];
                        // Fail: No space
                        kind.extend(read_bytes_before(reader, is_space, 8)?);
                        // Fail: No utf8 or ascii
                        let count = string_from_vec_ascii(kind)?;

                        // Fail: Only space
                        let mut kind = vec![read_byte_after(reader, is_space)?];
                        // Fail: No space
                        kind.extend(read_bytes_before(reader, is_space, 8)?);
                        // Fail: No utf8 or ascii
                        let value = string_from_vec_ascii(kind)?;

                        List { count, value }
                    };

                    // Fail: No newline or no utf8 or ascii
                    let name =
                        string_from_vec_ascii(read_bytes_before_newline(reader, 16)?)?;

                    properties.insert(name.to_owned(), Property { kind, name });
                },
                b"element " => {
                    let properties = Default::default();

                    // Fail: Only space
                    let mut name = vec![read_byte_after(reader, is_space)?];
                    // Fail: No space
                    name.extend(read_bytes_before(reader, is_space, 16)?);
                    // Fail: No utf8 or ascii
                    let name = string_from_vec_ascii(name)?;

                    // Fail: No newline or no utf8 or no ascii or not only digits
                    let size =
                        string_from_vec_ascii(read_bytes_before_newline(reader, 8)?)?
                            .parse::<usize>()?;

                    elements.insert(
                        name.to_owned(),
                        Element {
                            properties,
                            name,
                            size,
                        },
                    );
                },
                b"comment " | b"obj_info" => {
                    // Fail: No newline
                    drop(read_bytes_before_newline(reader, 64)?);
                },
                _ => Err(MissingSymbol(
                    "comment, element, end_header, obj_info, or property".into(),
                ))?,
            }
        }

        Ok(Header {
            format,
            version,
            elements,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn decode_on_another_cube_example() {
        use super::*;

        let source = &mut Cursor::new(include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
        ));

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
        let output = &header.elements.values().map(|e| e.size).collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![
            ("float".to_string().into(), "x".to_string()).into(),
            ("float".to_string().into(), "y".to_string()).into(),
            ("float".to_string().into(), "z".to_string()).into(),
            ("uchar".to_string().into(), "red".to_string()).into(),
            ("uchar".to_string().into(), "green".to_string()).into(),
            ("uchar".to_string().into(), "blue".to_string()).into(),
        ];
        let output = &header.elements["vertex"]
            .properties
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = &vec![(
            ("uchar".to_string().into(), "int".to_string()).into(),
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
            (("int".to_string().into()), "vertex1".to_string()).into(),
            (("int".to_string().into()), "vertex2".to_string()).into(),
            (("uchar".to_string().into()), "red".to_string()).into(),
            (("uchar".to_string().into()), "green".to_string()).into(),
            (("uchar".to_string().into()), "blue".to_string()).into(),
        ];
        let output = &header.elements["edge"]
            .properties
            .values()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(output, target);
    }
}
