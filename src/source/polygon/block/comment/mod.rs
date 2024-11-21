pub mod obj_info;

use std::fmt;

pub use super::*;
pub use obj_info::*;

/// ## Syntax
///
/// ```plaintext
/// <comment-block> :=
///     | <message> <newline>
///
/// <message> :=
///     | [{" "}] <ascii-string>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentBlock {
    pub message: AsciiString,
}

impl CommentBlock {
    #[inline]
    pub fn new<S: AsRef<[u8]>>(message: S) -> Result<Self, Error> {
        let message = message.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(err.into_source()).into_owned())
        })?;
        Ok(Self { message })
    }
}

impl Decoder for CommentBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut message = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<comment>".into()))?];
        message.extend(read_bytes_before_newline(reader, 64)?);
        let message = message.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(&err.into_source()).into_owned())
        })?;

        Ok(Self { message })
    }
}

impl Encoder for CommentBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(self.as_bytes())?;
        Ok(writer.write_all(NEWLINE)?)
    }
}

impl fmt::Debug for CommentBlock {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl ops::Deref for CommentBlock {
    type Target = AsciiString;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl ops::DerefMut for CommentBlock {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.message
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
