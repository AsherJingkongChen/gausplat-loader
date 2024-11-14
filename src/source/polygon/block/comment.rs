pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};

use crate::function::{read_byte_after, read_bytes_before_newline};
use std::{io::Read, ops::Deref};

/// ## Syntax
///
/// ```plaintext
/// <comment-block> :=
///    | [{" "}] <message> ["\r"] "\n"
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentBlock {
    pub message: AsciiString,
}

impl Decoder for CommentBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut message = vec![read_byte_after(reader, |b| b == b' ')?
            .ok_or_else(|| Error::MissingToken("<message>".into()))?];
        message.extend(read_bytes_before_newline(reader, 64)?);
        let message = message.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;

        Ok(Self { message })
    }
}

impl Deref for CommentBlock {
    type Target = AsciiString;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = b"Hello, World!\n";
        let reader = &mut Cursor::new(source);

        let target = source.len() - 1;
        let output = CommentBlock::decode(reader).unwrap().len();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"    ");

        CommentBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_message() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new("\u{ae}");
        CommentBlock::decode(source).unwrap_err();
    }
}
