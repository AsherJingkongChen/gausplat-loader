pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <format-block> :=
///     | "format " <format-variant> [{" "}] <version> <newline>
///
/// <version> :=
///     | <ascii-string>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
/// - [`FormatVariant`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatBlock {
    pub variant: FormatVariant,
    pub version: AsciiString,
}

/// ## Syntax
///
/// ```plaintext
/// <format-variant> :=
///     | [{" "}] <format> " "
///
/// <format> :=
///     | "ascii"
///     | "binary_big_endian"
///     | "binary_little_endian"
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormatVariant {
    #[default]
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl FormatBlock {
    pub const KEYWORD: &[u8; 7] = b"format ";
}

impl FormatVariant {
    pub const DOMAIN: [&str; 3] =
        ["ascii", "binary_big_endian", "binary_little_endian"];
}

impl Decoder for FormatBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        if read_any::<[u8; 7]>(reader)? != *Self::KEYWORD {
            Err(Error::MissingToken(
                String::from_utf8(Self::KEYWORD.into()).expect("Unreachable"),
            ))?;
        }

        let variant = FormatVariant::decode(reader)?;

        let mut version = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<version>".into()))?];
        version.extend(read_bytes_before_newline(reader, 16)?);
        let version = version.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;

        Ok(Self { variant, version })
    }
}

impl Decoder for FormatVariant {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut variant = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<format-variant>".into()))?];
        variant.extend(read_bytes_before(reader, is_space, 20)?);

        Ok(match variant.as_slice() {
            b"binary_little_endian" => Self::BinaryLittleEndian,
            b"ascii" => Self::Ascii,
            b"binary_big_endian" => Self::BinaryBigEndian,
            _ => Err(Error::InvalidPolygonFormatVariant(
                String::from_utf8_lossy(&variant).into_owned(),
            ))?,
        })
    }
}

impl Default for FormatBlock {
    #[inline]
    fn default() -> Self {
        let variant = Default::default();
        let version = "1.0".into_ascii_string().expect("Unreachable");
        Self { variant, version }
    }
}

impl Encoder for FormatBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        write_bytes(writer, Self::KEYWORD)?;

        self.variant.encode(writer)?;

        write_bytes(writer, self.version.as_bytes())?;
        write_bytes(writer, NEWLINE)
    }
}

impl Encoder for FormatVariant {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        write_bytes(
            writer,
            match self {
                Self::Ascii => b"ascii",
                Self::BinaryBigEndian => b"binary_big_endian",
                Self::BinaryLittleEndian => b"binary_little_endian",
            },
        )?;
        write_bytes(writer, SPACE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"format ascii 1.0\n");
        let target = FormatBlock {
            variant: FormatVariant::Ascii,
            version: "1.0".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"format binary_little_endian 1.0.1\n");
        let target = FormatBlock {
            variant: FormatVariant::BinaryLittleEndian,
            version: "1.0.1".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);

        let source =
            &mut Cursor::new(b"format    binary_big_endian private    \n");
        let target = FormatBlock {
            variant: FormatVariant::BinaryBigEndian,
            version: "private    ".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_invalid_block() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"format binary_big_endian     ");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format binary_middle_endian 1.0\n");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new("format ascii 1.0+\u{ae}\n".as_bytes());
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"formatascii 1.0\n");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ascii");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"form");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        FormatBlock::decode(source).unwrap_err();
    }

    #[test]
    fn default() {
        use super::*;

        let target = FormatVariant::default();
        let output = FormatBlock::default();
        assert_eq!(output.variant, target);

        let target = "1.0";
        assert_eq!(output.version, target);
    }
}
