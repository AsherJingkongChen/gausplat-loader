pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <element-block> :=
///     | [{" "}] <name> [{" "}] <size> <newline>
///
/// <name> :=
///     | <ascii-string> " "
///
/// <size> :=
///     | <u64>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
/// - [`u64`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElementBlock {
    pub name: AsciiString,
    pub size: u64,
}

impl Decoder for ElementBlock {
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

impl Default for ElementBlock {
    #[inline]
    fn default() -> Self {
        let name = "default".into_ascii_string().expect("Unreachable");
        let size = Default::default();
        Self { name, size }
    }
}

impl Encoder for ElementBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        write_bytes(writer, self.name.as_bytes())?;
        write_bytes(writer, SPACE)?;

        write_bytes(writer, self.size.to_string().as_bytes())?;
        write_bytes(writer, NEWLINE)
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
        let output = ElementBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = 7;
        assert_eq!(output.size, target);

        let source = &mut Cursor::new(b"rgb 888 100\n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"     point    ");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"vertex 3 \n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b" ");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_name() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new("\u{ae} 3\n".as_bytes());
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_u64_size() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"none -1\n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"unicode \x8e\xcd\n");
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn default() {
        use super::*;

        let target = "default";
        let output = ElementBlock::default();
        assert_eq!(output.name, target);

        let target = 0;
        assert_eq!(output.size, target);
    }
}
