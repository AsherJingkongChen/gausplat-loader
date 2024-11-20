pub mod meta;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use indexmap::{IndexMap, IndexSet};
pub use meta::*;

use super::impl_variant_matchers;
use crate::function::{
    decode::{
        is_space, read_byte_after, read_bytes_before,
        read_bytes_before_newline, read_bytes_const,
    },
    encode::{NEWLINE, SPACE},
};
use std::{
    io::{Read, Write},
    ops,
};

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
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Head {
    format: FormatMeta,
    inner: Vec<Meta>,
}

impl Head {
    pub const KEYWORDS: &[&str; 5] = &[
        "comment ",
        "element ",
        "end_header",
        "property ",
        "obj_info ",
    ];
    pub const SIGNATURE: &[u8; 3] = b"ply";

    #[inline]
    pub fn new(format: FormatMeta) -> Self {
        let inner = Vec::with_capacity(16);
        Self { format, inner }
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
        *self.format = variant;
    }

    #[inline]
    pub fn get_version(&self) -> &str {
        self.format.version.as_str()
    }

    #[inline]
    pub fn set_version<V: AsRef<[u8]>>(
        &mut self,
        version: V,
    ) -> Result<(), Error> {
        self.format.version =
            version.as_ref().into_ascii_string().map_err(|err| {
                Error::InvalidAscii(
                    String::from_utf8_lossy(err.into_source()).into_owned(),
                )
            })?;
        Ok(())
    }
}

macro_rules! impl_head_filtered_iterators {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl Head {
                $(
                    #[inline]
                    pub fn [<iter_ $variant:snake>](
                        &self
                    ) -> impl Iterator<Item = &[<$variant Meta>]> {
                        self.iter().filter_map(|meta| meta.[<as_ $variant:snake>]())
                    }

                    #[inline]
                    pub fn [<iter_ $variant:snake _mut>](
                        &mut self
                    ) -> impl Iterator<Item = &mut [<$variant Meta>]> {
                        self.iter_mut().filter_map(|meta| meta.[<as_ $variant:snake _mut>]())
                    }
                )*
            }
        }
    };
}
impl_head_filtered_iterators! { Comment, Element, ObjInfo, Property }

// Special filtered iterators
impl Head {
    #[inline]
    pub fn iter_element_then_property(
        &self
    ) -> impl Iterator<Item = (&ElementMeta, impl Iterator<Item = &PropertyMeta>)>
    {
        self.iter().enumerate().filter_map(|(index, meta)| {
            let element = meta.as_element()?;
            let properties = self
                .iter()
                .skip(index + 1)
                .take_while(|meta| !meta.is_element())
                .filter_map(|meta| meta.as_property());
            Some((element, properties))
        })
    }

    #[inline]
    pub fn iter_element_and_property(
        &self
    ) -> impl Iterator<Item = (&ElementMeta, &PropertyMeta)> {
        self.iter_element_then_property()
            .flat_map(|(element, properties)| {
                properties.map(move |property| (element, property))
            })
    }
}

macro_rules! impl_head_format_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl Head {
                $(
                    #[inline]
                    pub const fn [<is_format_ $variant:snake>](&self) -> bool {
                        matches!(self.format.variant, FormatMetaVariant::$variant)
                    }
                )*
            }
        }
    };
}
impl_head_format_matchers! { Ascii, BinaryBigEndian, BinaryLittleEndian }

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

        let mut head = Head::new(FormatMeta::decode(reader)?);
        
        let mut had_element = false;
        loop {
            let keyword_prefix = read_bytes_const(reader)?;
            match &keyword_prefix {
                b"pr" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"operty " => {
                            // Rejecting the misplaced property.
                            if !had_element {
                                Err(Error::MissingToken("element ".into()))?;
                            }

                            head.push(
                                Property(PropertyMeta::decode(reader)?).into(),
                            );

                            Ok(())
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"el" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"ement " => {
                            head.push(
                                Element(ElementMeta::decode(reader)?).into(),
                            );
                            had_element = true;
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
                            head.push(
                                Comment(CommentMeta::decode(reader)?).into(),
                            );
                            Ok(())
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"ob" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"j_info " => {
                            head.push(
                                ObjInfo(ObjInfoMeta::decode(reader)?).into(),
                            );
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

        Ok(head)
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

        self.iter().try_for_each(|meta| match &**meta {
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

impl ops::Deref for Head {
    type Target = Vec<Meta>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Head {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

        let target = Ascii;
        head.set_format(Ascii);
        let output = head.get_format();
        assert_eq!(output, target);

        let target = FormatMetaVariant::BinaryLittleEndian;
        head.set_format(BinaryLittleEndian);
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

        let target = 2;
        let output = head.iter().filter(|meta| meta.is_comment()).count();
        assert_eq!(output, target);

        head.retain(|meta| !meta.is_comment());

        let target = 0;
        let output = head.iter().filter(|meta| meta.is_comment()).count();
        assert_eq!(output, target);
    }
}
