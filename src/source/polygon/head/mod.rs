pub mod group;
pub mod meta;

pub use super::object::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use group::*;
pub use indexmap::IndexMap;
pub use meta::*;

use crate::function::{
    decode::{
        is_space, read_byte_after, read_bytes_before,
        read_bytes_before_newline, read_bytes_const,
    },
    encode::{write_bytes, NEWLINE, SPACE},
};
use std::io::{Read, Write};

/// ## Syntax
///
/// ```plaintext
/// <head> :=
///     | <start_header> <format-meta> [{<meta>}] <end_header>
///
/// <start_header> :=
///     | "ply" <newline>
///
/// <end_header> :=
///     | "end_header" <newline>
///
/// <meta> :=
///     | "comment " <comment-meta>
///     | "element " <element-meta>
///     | "property " <property-meta>
///     | "obj_info " <obj_info-meta>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`FormatMeta`]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Head {
    pub format: FormatMeta,
    pub group: Group,
    pub meta_map: IndexMap<Id, Meta>,
}

impl Head {
    pub const KEYWORD_DOMAIN: [&str; 5] = [
        "element ",
        "property ",
        "comment ",
        "obj_info ",
        "end_header",
    ];

    pub const SIGNATURE: &[u8; 3] = b"ply";
}

impl Decoder for Head {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use MetaVariant::*;

        if read_bytes_const(reader)? != *Self::SIGNATURE {
            Err(Error::MissingToken(
                String::from_utf8(Self::SIGNATURE.into()).expect("Unreachable"),
            ))?;
        }
        if !read_bytes_before_newline(reader, 0)?.is_empty() {
            Err(Error::MissingToken("<newline>".into()))?;
        }

        let format = FormatMeta::decode(reader)?;

        let mut meta_map = IndexMap::default();
        let mut group_builder = GroupBuilder::default();

        loop {
            let id = Id::new();

            let keyword_prefix = read_bytes_const(reader)?;
            let variant = match &keyword_prefix {
                b"pr" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"operty " => {
                            group_builder =
                            group_builder.add_property_id(id).ok_or_else(
                                || Error::MissingToken("element ".into()),
                            )?;
                            Ok(Property(PropertyMeta::decode(reader)?))
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"el" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"ement " => {
                            group_builder = group_builder.set_element_id(id);
                            Ok(Element(ElementMeta::decode(reader)?))
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"en" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"d_header" => {
                            match read_bytes_before_newline(reader, 0)?
                                .as_slice()
                            {
                                b"" => break,
                                keyword_suffix_rest => {
                                    Err([&keyword_suffix, keyword_suffix_rest]
                                        .concat()
                                        .into())
                                },
                            }
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"co" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"mment " => Ok(Comment(CommentMeta::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"ob" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"j_info " => Ok(ObjInfo(ObjInfoMeta::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                _ => Err(Default::default()),
            }
            .map_err(|keyword_suffix: Box<[u8]>| {
                let keyword = &[&keyword_prefix[..], &keyword_suffix].concat();
                Error::InvalidPolygonKeyword(
                    String::from_utf8_lossy(keyword).into_owned(),
                )
            })?;

            meta_map.insert(id, Meta { id, variant });
        }

        let group = group_builder.build();

        Ok(Self {
            format,
            group,
            meta_map,
        })
    }
}

impl Encoder for Head {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        use MetaVariant::*;

        write_bytes(writer, b"ply")?;
        write_bytes(writer, NEWLINE)?;

        self.format.encode(writer)?;

        self.meta_map
            .values()
            .try_for_each(|meta| match &meta.variant {
                Property(meta) => {
                    write_bytes(writer, b"property ")?;
                    meta.encode(writer)
                },
                Element(meta) => {
                    write_bytes(writer, b"element ")?;
                    meta.encode(writer)
                },
                Comment(meta) => {
                    write_bytes(writer, b"comment ")?;
                    meta.encode(writer)
                },
                ObjInfo(meta) => {
                    write_bytes(writer, b"obj_info ")?;
                    meta.encode(writer)
                },
            })?;

        write_bytes(writer, b"end_header")?;
        write_bytes(writer, NEWLINE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_and_encode() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/valid-keyword.ascii.ply"
        );

        let reader = &mut Cursor::new(source);
        let output = Head::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        output.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_misplaced_property() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/misplaced-property.ascii.ply"
        );

        let reader = &mut Cursor::new(source);
        Head::decode(reader).unwrap_err();
    }

    #[test]
    fn decode_on_empty_head() {
        use super::*;
        use std::io::{Cursor, ErrorKind};

        let source = include_bytes!(
            "../../../../examples/data/polygon/empty-head.ascii.ply"
        );

        let reader = &mut Cursor::new(source);
        let target = Head::default();
        let output = Head::decode(reader).unwrap();
        assert_eq!(output, target);

        let target = ErrorKind::UnexpectedEof;
        let output = reader.read_exact(&mut [0; 1]).unwrap_err().kind();
        assert_eq!(output, target);

        let reader = &mut Cursor::new(&source[..source.len() - 1]);
        let target = Head::default();
        let output = Head::decode(reader).unwrap();
        assert_eq!(output, target);

        let reader = &mut Cursor::new(
            [&source[..source.len() - 1], b" not newline"].concat(),
        );
        Head::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply\nend_header\n");
        Head::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply");
        Head::decode(reader).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_signature() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/empty-head.ascii.ply"
        );

        let reader = &mut Cursor::new([b"plh", &source[3..]].concat());
        Head::decode(reader).unwrap_err();

        let reader = &mut Cursor::new([b"ply\r\n", &source[4..]].concat());
        let target = Head::default();
        let output = Head::decode(reader).unwrap();
        assert_eq!(output, target);
    }
}
