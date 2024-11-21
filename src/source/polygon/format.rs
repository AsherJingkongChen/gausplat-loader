pub use super::*;
pub use FormatVariant::*;

/// ## Syntax
///
/// ```plaintext
/// <format> :=
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
pub struct Format {
    pub variant: FormatVariant,
    pub version: AsciiString,
}

/// ## Syntax
///
/// ```plaintext
/// <format-variant> :=
///     | [{" "}] <format-type> " "
///
/// <format-type> :=
///     | "ascii"
///     | "binary_big_endian"
///     | "binary_little_endian"
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FormatVariant {
    #[default]
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl Format {
    pub const KEYWORD: &[u8; 7] = b"format ";

    #[inline]
    pub fn new<V: AsRef<[u8]>>(
        variant: FormatVariant,
        version: V,
    ) -> Result<Self, Error> {
        let version = version.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(err.into_source()).into_owned())
        })?;
        Ok(Self { variant, version })
    }
}

macro_rules! impl_variant_checkers {
    ($subject:ident, $( $variant:ident ),* ) => {
        paste::paste! {
            impl [<$subject Variant>] {
                $(
                    #[inline]
                    pub const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, [<$subject Variant>]::$variant)
                    }
                )*
            }
        }
    };
}
impl_variant_checkers! { Format, Ascii, BinaryBigEndian, BinaryLittleEndian }

impl FormatVariant {
    pub const DOMAIN: [&str; 3] = ["ascii", "binary_big_endian", "binary_little_endian"];

    #[inline]
    pub const fn is_binary_native_endian(&self) -> bool {
        match self {
            BinaryLittleEndian => cfg!(target_endian = "little"),
            Ascii => false,
            BinaryBigEndian => cfg!(target_endian = "big"),
        }
    }
}

impl Decoder for Format {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        if read_bytes_const(reader)? != *Self::KEYWORD {
            // SAFETY: This is a UTF-8 string literal.
            Err(Error::MissingToken(unsafe {
                String::from_utf8(Self::KEYWORD.into()).unwrap_unchecked()
            }))?;
        }

        let variant = FormatVariant::decode(reader)?;

        let mut version = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<version>".into()))?];
        version.extend(read_bytes_before_newline(reader, 16)?);
        let version = version.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(&err.into_source()).into_owned())
        })?;

        Ok(Self { variant, version })
    }
}

impl Decoder for FormatVariant {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut variant = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<format-block-variant>".into()))?];
        variant.extend(read_bytes_before(reader, is_space, 20)?);

        Ok(match variant.as_slice() {
            b"binary_little_endian" => BinaryLittleEndian,
            b"ascii" => Ascii,
            b"binary_big_endian" => BinaryBigEndian,
            _ => Err(Error::InvalidPolygonFormatVariant(
                String::from_utf8_lossy(&variant).into_owned(),
            ))?,
        })
    }
}

impl Default for Format {
    #[inline]
    fn default() -> Self {
        FormatVariant::default().into()
    }
}

impl Encoder for Format {
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

impl Encoder for FormatVariant {
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

impl From<FormatVariant> for Format {
    #[inline]
    fn from(variant: FormatVariant) -> Self {
        // SAFETY: This is an ASCII string literal.
        let version = unsafe { "1.0".into_ascii_string().unwrap_unchecked() };
        Self { variant, version }
    }
}

impl ops::Deref for Format {
    type Target = FormatVariant;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl ops::DerefMut for Format {
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
        let target = Format {
            variant: Ascii,
            version: "1.0".into_ascii_string().unwrap(),
        };
        let output = Format::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"format binary_little_endian 1.0.1\n");
        let target = Format {
            variant: BinaryLittleEndian,
            version: "1.0.1".into_ascii_string().unwrap(),
        };
        let output = Format::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"format    binary_big_endian private    \n");
        let target = Format {
            variant: BinaryBigEndian,
            version: "private    ".into_ascii_string().unwrap(),
        };
        let output = Format::decode(source).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_invalid_block() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"format binary_big_endian     ");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format binary_middle_endian 1.0\n");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new("format ascii 1.0+\u{ae}\n".as_bytes());
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"formatascii 1.0\n");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ascii");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"format ");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"fromat ");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"form");
        Format::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        Format::decode(source).unwrap_err();
    }

    #[test]
    fn encode_on_slice() {
        use super::*;

        Format::default().encode(&mut &mut [0][..]).unwrap_err();

        let target = b"format ascii 1.0\n";
        let mut output = Vec::new();
        Format::default().encode(&mut output).unwrap();
        assert_eq!(output.as_slice(), target);
    }

    #[test]
    fn new() {
        use super::*;

        let target = Format {
            variant: Ascii,
            version: "1.0".into_ascii_string().unwrap(),
        };
        let output = Format::new(Ascii, "1.0").unwrap();
        assert_eq!(output, target);

        let target = false;
        let output = output.is_binary_native_endian();
        assert_eq!(output, target);

        Format::new(Default::default(), "\u{ae}").unwrap_err();
    }
}
