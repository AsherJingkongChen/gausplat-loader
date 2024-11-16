pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <format-block> :=
///     | <format-variant> [{" "}] <version> ["\r"] "\n"
///
/// <version> :=
///     | <ascii-string>
/// ```
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

impl Decoder for FormatBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
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
            _ => Err(Error::UnknownPolygonPropertyKind(
                String::from_utf8_lossy(&variant).into_owned(),
            ))?,
        })
    }
}

impl Default for FormatBlock {
    #[inline]
    fn default() -> Self {
        let variant = Default::default();
        // SAFETY: This is an ASCII string literal.
        let version = unsafe { "1.0".into_ascii_string_unchecked() };
        Self { variant, version }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"ascii 1.0\n");
        let target = FormatBlock {
            variant: FormatVariant::Ascii,
            version: "1.0".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"binary_little_endian 1.0.1\n");
        let target = FormatBlock {
            variant: FormatVariant::BinaryLittleEndian,
            version: "1.0.1".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"    binary_big_endian private    \n");
        let target = FormatBlock {
            variant: FormatVariant::BinaryBigEndian,
            version: "private    ".into_ascii_string().unwrap(),
        };
        let output = FormatBlock::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"binary_big_endian     ");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"binary_middle_endian\n");
        FormatBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new("ascii 1.0+\u{ae}\n".as_bytes());
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
