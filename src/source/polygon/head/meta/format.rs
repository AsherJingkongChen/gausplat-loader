pub use super::*;
pub use FormatMetaVariant::*;

/// ## Syntax
///
/// ```plaintext
/// <format-meta> :=
///     | "format " <format-meta-variant> [{" "}] <version> <newline>
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
/// - [`FormatMetaVariant`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatMeta {
    pub variant: FormatMetaVariant,
    pub version: AsciiString,
}

/// ## Syntax
///
/// ```plaintext
/// <format-meta-variant> :=
///     | [{" "}] <format> " "
///
/// <format> :=
///     | "ascii"
///     | "binary_big_endian"
///     | "binary_little_endian"
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormatMetaVariant {
    #[default]
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl FormatMeta {
    pub const KEYWORD: &[u8; 7] = b"format ";
}

impl FormatMetaVariant {
    pub const DOMAIN: [&str; 3] =
        ["ascii", "binary_big_endian", "binary_little_endian"];
}

impl Decoder for FormatMeta {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        if read_bytes_const(reader)? != *Self::KEYWORD {
            Err(Error::MissingToken(
                String::from_utf8(Self::KEYWORD.into()).expect("Unreachable"),
            ))?;
        }

        let variant = FormatMetaVariant::decode(reader)?;

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

impl Decoder for FormatMetaVariant {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut variant =
            vec![read_byte_after(reader, is_space)?.ok_or_else(|| {
                Error::MissingToken("<format-meta-variant>".into())
            })?];
        variant.extend(read_bytes_before(reader, is_space, 20)?);

        Ok(match variant.as_slice() {
            b"binary_little_endian" => BinaryLittleEndian,
            b"ascii" => Ascii,
            b"binary_big_endian" => BinaryBigEndian,
            _ => Err(Error::InvalidPolygonFormatMetaVariant(
                String::from_utf8_lossy(&variant).into_owned(),
            ))?,
        })
    }
}

impl Default for FormatMeta {
    #[inline]
    fn default() -> Self {
        let variant = Default::default();
        let version = "1.0".into_ascii_string().expect("Unreachable");
        Self { variant, version }
    }
}

impl Encoder for FormatMeta {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(Self::KEYWORD)?;

        self.variant.encode(writer)?;

        writer.write_all(self.version.as_bytes())?;
        Ok(writer.write_all(NEWLINE)?)
    }
}

impl Encoder for FormatMetaVariant {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(match self {
            BinaryLittleEndian => b"binary_little_endian",
            Ascii => b"ascii",
            BinaryBigEndian => b"binary_big_endian",
        })?;
        Ok(writer.write_all(SPACE)?)
    }
}

impl ops::Deref for FormatMeta {
    type Target = FormatMetaVariant;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl ops::DerefMut for FormatMeta {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"format ascii 1.0\n");
        let target = FormatMeta {
            variant: Ascii,
            version: "1.0".into_ascii_string().unwrap(),
        };
        let output = FormatMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"format binary_little_endian 1.0.1\n");
        let target = FormatMeta {
            variant: BinaryLittleEndian,
            version: "1.0.1".into_ascii_string().unwrap(),
        };
        let output = FormatMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source =
            &mut Cursor::new(b"format    binary_big_endian private    \n");
        let target = FormatMeta {
            variant: BinaryBigEndian,
            version: "private    ".into_ascii_string().unwrap(),
        };
        let output = FormatMeta::decode(source).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_invalid_meta() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"format binary_big_endian     ");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format binary_middle_endian 1.0\n");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new("format ascii 1.0+\u{ae}\n".as_bytes());
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"formatascii 1.0\n");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ascii");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"fromat ");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"form");
        FormatMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        FormatMeta::decode(source).unwrap_err();
    }
}
