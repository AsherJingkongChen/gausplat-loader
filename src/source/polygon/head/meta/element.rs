pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <element-meta> :=
///     | [{" "}] <name> [{" "}] <size> <newline>
///
/// <name> :=
///     | <ascii-string> " "
///
/// <size> :=
///     | <usize>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
/// - [`usize`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElementMeta {
    pub name: AsciiString,
    pub size: usize,
}

impl Decoder for ElementMeta {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut name = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<name>".into()))?];
        name.extend(read_bytes_before(reader, is_space, 16)?);
        let name = name.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;

        let mut size = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<size>".into()))?];
        size.extend(read_bytes_before_newline(reader, 16)?);
        let size = std::str::from_utf8(&size)
            .map_err(|_| {
                Error::InvalidUtf8(String::from_utf8_lossy(&size).into_owned())
            })?
            .parse()?;

        Ok(Self { name, size })
    }
}

impl Default for ElementMeta {
    #[inline]
    fn default() -> Self {
        let name = "default".into_ascii_string().expect("Unreachable");
        let size = Default::default();
        Self { name, size }
    }
}

impl Encoder for ElementMeta {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(SPACE)?;

        writer.write_all(self.size.to_string().as_bytes())?;
        Ok(writer.write_all(NEWLINE)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"color 7\n");
        let target = "color";
        let output = ElementMeta::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = 7;
        assert_eq!(output.size, target);

        let source = &mut Cursor::new(b"rgb 888 100\n");
        ElementMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"     point    ");
        ElementMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"vertex 3 \n");
        ElementMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b" ");
        ElementMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        ElementMeta::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_name() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new("\u{ae} 3\n".as_bytes());
        ElementMeta::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_usize_size() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"none -1\n");
        ElementMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"unicode \x8e\xcd\n");
        ElementMeta::decode(source).unwrap_err();
    }

    #[test]
    fn default() {
        use super::*;

        let target = "default";
        let output = ElementMeta::default();
        assert_eq!(output.name, target);

        let target = 0;
        assert_eq!(output.size, target);
    }
}
