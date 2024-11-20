pub mod obj_info;

use std::fmt;

pub use super::*;
pub use obj_info::*;

/// ## Syntax
///
/// ```plaintext
/// <comment-meta> :=
///     | [{" "}] <comment> <newline>
///
/// <comment> :=
///     | <ascii-string>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentMeta {
    inner: AsciiString,
}

impl CommentMeta {
    #[inline]
    pub fn new<S: AsRef<[u8]>>(inner: S) -> Result<Self, Error> {
        let inner = inner.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(err.into_source()).into_owned())
        })?;
        Ok(Self { inner })
    }

    #[inline]
    pub fn into_inner(self) -> AsciiString {
        self.inner
    }
}

impl Decoder for CommentMeta {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut inner = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<comment>".into()))?];
        inner.extend(read_bytes_before_newline(reader, 64)?);
        let inner = inner.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(&err.into_source()).into_owned())
        })?;

        Ok(Self { inner })
    }
}

impl Encoder for CommentMeta {
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

impl fmt::Debug for CommentMeta {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl From<AsciiString> for CommentMeta {
    #[inline]
    fn from(inner: AsciiString) -> Self {
        Self { inner }
    }
}

impl ops::Deref for CommentMeta {
    type Target = AsciiString;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for CommentMeta {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
        let output = CommentMeta::decode(reader).unwrap().len();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"    ");

        CommentMeta::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_message() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new("\u{ae}");
        CommentMeta::decode(source).unwrap_err();
    }
}
