pub mod obj_info;

pub use super::*;
pub use obj_info::*;

/// ## Syntax
///
/// ```plaintext
/// <comment-block> :=
///     | [{" "}] <message> <newline>
///
/// <message> :=
///     | <ascii-string>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentBlock {
    pub message: AsciiString,
}

impl Decoder for CommentBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut message = vec![read_byte_after(reader, is_space)?
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

impl Default for CommentBlock {
    #[inline]
    fn default() -> Self {
        // SAFETY: This is an ASCII string literal.
        let message = unsafe { "default".into_ascii_string_unchecked() };
        Self { message }
    }
}

impl Encoder for CommentBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        write_bytes(writer, self.message.as_bytes())?;
        write_bytes(writer, NEWLINE)
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
        let output = CommentBlock::decode(reader).unwrap().message.len();
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

    #[test]
    fn default() {
        use super::*;

        let target = "default";
        let output = CommentBlock::default().message;
        assert_eq!(output, target);
    }
}
