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

use super::{impl_map_accessors, impl_variant_matchers};
use crate::function::{
    decode::{
        is_space, read_byte_after, read_bytes_before,
        read_bytes_before_newline, read_bytes_const,
    },
    encode::{NEWLINE, SPACE},
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
/// <meta> :=
///     | "comment " <comment-meta>
///     | "element " <element-meta>
///     | "obj_info " <obj_info-meta>
///     | "property " <property-meta>
///
/// <end_header> :=
///     | "end_header" <newline>
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
    format: FormatMeta,
    group: Group,
    meta_map: IndexMap<Id, Meta>,
}

impl Head {
    pub const KEYWORDS: [&str; 5] = [
        "comment ",
        "element ",
        "end_header",
        "property ",
        "obj_info ",
    ];
    pub const SIGNATURE: &[u8; 3] = b"ply";

    impl_map_accessors!(Meta, Comment, Element, ObjInfo, Property);

    pub fn remove_comment(
        &mut self,
        id: &Id,
    ) -> Option<Meta> {
        self.meta_map.shift_remove(id)
    }

    #[inline]
    pub const fn get_format(&self) -> FormatMetaVariant {
        self.format.variant
    }

    #[inline]
    pub fn set_format(
        &mut self,
        variant: FormatMetaVariant,
    ) {
        let is_same_endian = match variant {
            BinaryLittleEndian => self.is_format_binary_little_endian(),
            Ascii => self.is_format_ascii(),
            BinaryBigEndian => self.is_format_binary_big_endian(),
        };
        if !is_same_endian {
            log::warn!(
                target: "gausplat-loader::source::polygon::head",
                "The format is changed to {variant:?}. The data may be corrupted.",
            );
        }

        self.format.variant = variant;
    }

    #[inline]
    pub fn get_version(&self) -> &str {
        self.format.version.as_str()
    }

    #[inline]
    pub fn set_version<S: AsRef<[u8]>>(
        &mut self,
        version: S,
    ) -> Result<(), Error> {
        self.format.version =
            version.as_ref().into_ascii_string().map_err(|err| {
                Error::InvalidAscii(
                    String::from_utf8_lossy(err.into_source()).into_owned(),
                )
            })?;
        Ok(())
    }

    #[inline]
    pub fn iter_element_and_property(
        &self
    ) -> impl Iterator<
        Item = (
            (&Id, &ElementMeta),
            impl Iterator<Item = (&Id, &PropertyMeta)>,
        ),
    > {
        self.group.iter_element_id_and_property_ids().map(
            |(element_id, property_ids)| {
                let element = self
                    .meta_map
                    .get(element_id)
                    .expect("Unreachable")
                    .variant
                    .as_element()
                    .expect("Unreachable");

                let properties = property_ids.iter().map(|property_id| {
                    (
                        property_id,
                        self.meta_map
                            .get(property_id)
                            .expect("Unreachable")
                            .variant
                            .as_property()
                            .expect("Unreachable"),
                    )
                });

                ((element_id, element), properties)
            },
        )
    }
}

impl_head_format_matchers_and_setters!(
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian
);
macro_rules! impl_head_format_matchers_and_setters {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl Head {
                $(
                    #[inline]
                    pub const fn [<is_format_ $variant:snake>](&self) -> bool {
                        matches!(self.format.variant, FormatMetaVariant::$variant)
                    }

                    #[inline]
                    pub fn [<set_format_ $variant:snake>](&mut self) {
                        self.set_format(FormatMetaVariant::$variant);
                    }
                )*
            }
        }
    };
}
use impl_head_format_matchers_and_setters;

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

        let mut group = GroupBuilder::default();
        let mut meta_map = IndexMap::with_capacity(16);

        loop {
            let keyword_prefix = read_bytes_const(reader)?;
            match &keyword_prefix {
                b"pr" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"operty " => {
                            // NOTE: Inserting the most recent element.
                            if let Some(variant) =
                                group.take_element().map(Element)
                            {
                                let id = Id::new();
                                group.set_element_id(id);
                                meta_map.insert(id, Meta { id, variant });
                            }

                            // NOTE: Rejecting the misplaced property.
                            let variant =
                                Property(PropertyMeta::decode(reader)?);
                            let id = Id::new();
                            group.add_property_id(id).ok_or_else(|| {
                                Error::MissingToken("element ".into())
                            })?;
                            meta_map.insert(id, Meta { id, variant });

                            Ok(())
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"el" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"ement " => {
                            group.set_element(ElementMeta::decode(reader)?);
                            Ok(())
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
                        b"mment " => {
                            let variant = Comment(CommentMeta::decode(reader)?);
                            let id = Id::new();
                            meta_map.insert(id, Meta { id, variant });
                            Ok(())
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"ob" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"j_info " => {
                            let variant = ObjInfo(ObjInfoMeta::decode(reader)?);
                            let id = Id::new();
                            meta_map.insert(id, Meta { id, variant });
                            Ok(())
                        },
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
        }

        let group = group.build();

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

        writer.write_all(b"ply")?;
        writer.write_all(NEWLINE)?;

        self.format.encode(writer)?;

        self.meta_map
            .values()
            .try_for_each(|meta| match &meta.variant {
                Property(meta) => {
                    writer.write_all(b"property ")?;
                    meta.encode(writer)
                },
                Element(meta) => {
                    writer.write_all(b"element ")?;
                    meta.encode(writer)
                },
                Comment(meta) => {
                    writer.write_all(b"comment ")?;
                    meta.encode(writer)
                },
                ObjInfo(meta) => {
                    writer.write_all(b"obj_info ")?;
                    meta.encode(writer)
                },
            })?;

        writer.write_all(b"end_header")?;
        Ok(writer.write_all(NEWLINE)?)
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

    #[test]
    fn get_and_set_format() {
        use super::*;

        let mut head = Head::default();

        let target = FormatMetaVariant::Ascii;
        let output = head.get_format();
        assert_eq!(output, target);

        let target = FormatMetaVariant::BinaryBigEndian;
        head.set_format(target);
        let output = head.get_format();
        assert_eq!(output, target);

        let target = FormatMetaVariant::Ascii;
        head.set_format_ascii();
        let output = head.get_format();
        assert_eq!(output, target);

        let target = FormatMetaVariant::BinaryLittleEndian;
        head.set_format_binary_little_endian();
        let output = head.get_format();
        assert_eq!(output, target);
    }

    #[test]
    fn get_and_set_version() {
        use super::*;

        let mut head = Head::default();

        let target = "1.0";
        let output = head.get_version();
        assert_eq!(output, target);

        let target = "1.1.10";
        head.set_version(target).unwrap();
        let output = head.get_version();
        assert_eq!(output, target);

        head.set_version("private+\u{ae}").unwrap_err();
    }

    #[test]
    fn remove_comment_on_all() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
        );
        let reader = &mut Cursor::new(source);
        let mut head = Head::decode(reader).unwrap();

        let target = true;
        let output = head.iter_comment().next().is_some();
        assert_eq!(output, target);

        head.iter_comment()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|id| {
                head.remove_comment(&id).unwrap();
            });
        
        let target = false;
        let output = head.iter_comment().next().is_some();
        assert_eq!(output, target);
    }
}
