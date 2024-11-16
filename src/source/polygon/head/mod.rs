pub mod block;

pub use super::group::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use block::*;
pub use indexmap::IndexMap;

use crate::function::{
    is_space, read_any, read_byte_after, read_bytes, read_bytes_before,
    read_bytes_before_newline,
};
use std::io::Read;

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
}

impl Decoder for Head {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use HeadBlockVariant::*;

        if &read_any::<[u8; 3]>(reader)? != b"ply" {
            Err(Error::MissingToken("ply".into()))?;
        }
        if read_bytes_before_newline(reader, 0)?.len() != 0 {
            Err(Error::MissingToken("<newline>".into()))?;
        }

        let format = FormatBlock::decode(reader)?;

        let mut blocks = IndexMap::default();
        loop {
            let mut keyword = read_bytes(reader, 2)?;
            let variant = match keyword.as_slice() {
                b"en" => {
                    let mut keyword_suffix = read_bytes(reader, 8)?;
                    match keyword_suffix.as_slice() {
                        b"d_header" => {
                            match read_bytes_before_newline(reader, 0)?
                                .as_slice()
                            {
                                b"" => break,
                                keyword_suffix_rest => {
                                    keyword_suffix.extend(keyword_suffix_rest);
                                    Err(keyword_suffix)
                                },
                            }
                        },
                        _ => Err(keyword_suffix),
                    }
                },
                b"co" => {
                    let keyword_suffix = read_bytes(reader, 6)?;
                    match keyword_suffix.as_slice() {
                        b"mment " => Ok(Comment(CommentBlock::decode(reader)?)),
                        _ => Err(keyword_suffix),
                    }
                },
                b"el" => {
                    let keyword_suffix = read_bytes(reader, 6)?;
                    match keyword_suffix.as_slice() {
                        b"ement " => Ok(Element(ElementBlock::decode(reader)?)),
                        _ => Err(keyword_suffix),
                    }
                },
                b"pr" => {
                    let keyword_suffix = read_bytes(reader, 7)?;
                    match keyword_suffix.as_slice() {
                        b"operty " => {
                            Ok(Property(PropertyBlock::decode(reader)?))
                        },
                        _ => Err(keyword_suffix),
                    }
                },
                b"ob" => {
                    let keyword_suffix = read_bytes(reader, 7)?;
                    match keyword_suffix.as_slice() {
                        b"j_info " => {
                            Ok(ObjInfo(ObjInfoBlock::decode(reader)?))
                        },
                        _ => Err(keyword_suffix),
                    }
                },
                _ => Err(Default::default()),
            }
            .map_err(|keyword_suffix| {
                keyword.extend(keyword_suffix);
                Error::InvalidPolygonKeyword(
                    String::from_utf8_lossy(&keyword).into_owned(),
                )
            })?;

            let id = Id::default();

            blocks.insert(id, HeadBlock { id, variant });
        }

        Ok(Self { format, blocks })
    }
}

#[cfg(test)]
mod tests {
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
