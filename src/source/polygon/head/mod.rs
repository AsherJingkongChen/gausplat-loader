pub mod block;

pub use super::group::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use block::*;
pub use indexmap::IndexMap;

use crate::function::{
    decode::{
        is_space, read_any, read_byte_after, read_bytes_before,
        read_bytes_before_newline,
    },
    encode::{write_bytes, NEWLINE, SPACE},
};
use std::io::{Read, Write};

/// ## Syntax
///
/// ```plaintext
/// <head> :=
///     | <start_header> <format-block> [{<head-block>}] <end_header>
///
/// <start_header> :=
///     | "ply" <newline>
///
/// <end_header> :=
///     | "end_header" <newline>
///
/// <head-block> :=
///     | "comment " <comment-block>
///     | "element " <element-block>
///     | "property " <property-block>
///     | "obj_info " <obj_info-block>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`FormatBlock`]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Head {
    pub format: FormatBlock,
    pub blocks: IndexMap<Id, HeadBlock>,
}

impl Head {
    pub const KEYWORD_DOMAIN: [&str; 5] =
        ["element", "property", "comment", "obj_info", "end_header"];

    pub const SIGNATURE: &[u8; 3] = b"ply";
}

impl Decoder for Head {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use HeadBlockVariant::*;

        if &read_any::<[u8; 3]>(reader)? != Self::SIGNATURE {
            Err(Error::MissingToken("ply".into()))?;
        }
        if !read_bytes_before_newline(reader, 0)?.is_empty() {
            Err(Error::MissingToken("<newline>".into()))?;
        }

        let format = FormatBlock::decode(reader)?;

        let mut blocks = IndexMap::default();
        loop {
            let keyword_prefix = &read_any::<[u8; 2]>(reader)?;
            let variant = match keyword_prefix {
                b"en" => {
                    let keyword_suffix = &read_any::<[u8; 8]>(reader)?;
                    match keyword_suffix {
                        b"d_header" => {
                            match read_bytes_before_newline(reader, 0)?
                                .as_slice()
                            {
                                b"" => break,
                                keyword_suffix_rest => {
                                    Err([keyword_suffix, keyword_suffix_rest]
                                        .concat()
                                        .to_vec())
                                },
                            }
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"co" => {
                    let keyword_suffix = &read_any::<[u8; 6]>(reader)?;
                    match keyword_suffix {
                        b"mment " => Ok(Comment(CommentBlock::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"el" => {
                    let keyword_suffix = &read_any::<[u8; 6]>(reader)?;
                    match keyword_suffix {
                        b"ement " => Ok(Element(ElementBlock::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"pr" => {
                    let keyword_suffix = &read_any::<[u8; 7]>(reader)?;
                    match keyword_suffix {
                        b"operty " => {
                            Ok(Property(PropertyBlock::decode(reader)?))
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"ob" => {
                    let keyword_suffix = &read_any::<[u8; 7]>(reader)?;
                    match keyword_suffix {
                        b"j_info " => {
                            Ok(ObjInfo(ObjInfoBlock::decode(reader)?))
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                _ => Err(Default::default()),
            }
            .map_err(|keyword_suffix| {
                let keyword =
                    &[keyword_prefix, keyword_suffix.as_slice()].concat();
                Error::InvalidPolygonKeyword(
                    String::from_utf8_lossy(keyword).into_owned(),
                )
            })?;

            let id = Id::default();

            blocks.insert(id, HeadBlock { id, variant });
        }

        Ok(Self { format, blocks })
    }
}

impl Encoder for Head {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        use HeadBlockVariant::*;

        write_bytes(writer, b"ply")?;
        write_bytes(writer, NEWLINE)?;

        self.format.encode(writer)?;

        self.blocks
            .values()
            .try_for_each(|block| match &block.variant {
                Comment(block) => {
                    write_bytes(writer, b"comment ")?;
                    block.encode(writer)
                },
                Element(block) => {
                    write_bytes(writer, b"element ")?;
                    block.encode(writer)
                },
                Property(block) => {
                    write_bytes(writer, b"property ")?;
                    block.encode(writer)
                },
                ObjInfo(block) => {
                    write_bytes(writer, b"obj_info ")?;
                    block.encode(writer)
                },
            })?;

        write_bytes(writer, b"end_header")?;
        write_bytes(writer, NEWLINE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/valid-block-ascii.ply"
        );

        let reader = &mut Cursor::new(source);
        // let target = todo!();
        let output = Head::decode(reader).unwrap();
        println!("{:#?}", output);
    }

    #[test]
    fn decode_on_empty_head() {
        use super::*;
        use std::io::{Cursor, ErrorKind};

        let source = include_bytes!(
            "../../../../examples/data/polygon/empty-head-ascii.ply"
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
            "../../../../examples/data/polygon/empty-head-ascii.ply"
        );

        let reader = &mut Cursor::new([b"plh", &source[3..]].concat());
        Head::decode(reader).unwrap_err();

        let reader = &mut Cursor::new([b"ply\r\n", &source[4..]].concat());
        let target = Head::default();
        let output = Head::decode(reader).unwrap();
        assert_eq!(output, target);
    }
}
