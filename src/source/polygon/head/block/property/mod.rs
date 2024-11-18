pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

/// ## Syntax
///
/// ```plaintext
/// <property-block> :=
///     | <property-block-variant> [{" "}] <name> <newline>
///
/// <name> :=
///     | <ascii-string>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
/// - [`PropertyBlockVariant`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PropertyBlock {
    pub name: AsciiString,
    pub variant: PropertyBlockVariant,
}

/// ## Syntax
///
/// ```plaintext
/// <property-block-variant> :=
///     | [{" "}] "list" " " <list-property-block>
///     | <scalar-property-block>
/// ```
///
/// ### Syntax Reference
///
/// - [`ListPropertyBlock`]
/// - [`ScalarPropertyBlock`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PropertyBlockVariant {
    List(ListPropertyBlock),
    Scalar(ScalarPropertyBlock),
}

impl_property_block_variant_matchers!(List, Scalar);

impl Decoder for PropertyBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let variant = PropertyBlockVariant::decode(reader)?;

        let mut name = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<name>".into()))?];
        name.extend(read_bytes_before_newline(reader, 16)?);
        let name = name.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;

        Ok(Self { name, variant })
    }
}

impl Decoder for PropertyBlockVariant {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let list_or_scalar = ScalarPropertyBlock::decode(reader)?;
        Ok(match list_or_scalar.kind.as_bytes() {
            b"list" => Self::List(ListPropertyBlock::decode(reader)?),
            _ => Self::Scalar(list_or_scalar),
        })
    }
}

impl Default for PropertyBlock {
    #[inline]
    fn default() -> Self {
        let name = "default".into_ascii_string().expect("Unreachable");
        let variant = Default::default();
        Self { name, variant }
    }
}

impl Default for PropertyBlockVariant {
    #[inline]
    fn default() -> Self {
        Self::Scalar(Default::default())
    }
}

impl Encoder for PropertyBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        self.variant.encode(writer)?;
        write_bytes(writer, self.name.as_bytes())?;
        write_bytes(writer, NEWLINE)
    }
}

impl Encoder for PropertyBlockVariant {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        match self {
            Self::List(list) => {
                write_bytes(writer, b"list ")?;
                list.encode(writer)
            },
            Self::Scalar(scalar) => scalar.encode(writer),
        }
    }
}

macro_rules! impl_property_block_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl PropertyBlockVariant {
                $(
                    #[inline]
                    pub fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake>](&self) -> Option<&[<$variant PropertyBlock>]> {
                        match self {
                            Self::$variant(block) => Some(block),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_property_block_variant_matchers;

#[cfg(test)]
mod tests {
    #[test]
    fn decode_list() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"list uchar int vertex_indices\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "vertex_indices";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::List(ListPropertyBlock {
            count: UCHAR.to_owned(),
            entry: INT.to_owned(),
        });
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"list ushort uint    point_indices\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "point_indices";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::List(ListPropertyBlock {
            count: USHORT.to_owned(),
            entry: UINT.to_owned(),
        });
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"listuchar int vertex_indices\n");
        PropertyBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_scalar() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"float 32x\r\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "32x";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(FLOAT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"float32 x\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "x";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(FLOAT32.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "y";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "y";
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y    \r\n");
        let target = "y    ";
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"         int y\n");
        let target = "y";
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"uchar  \n");
        let target = "\n";
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyBlockVariant::Scalar(UCHAR.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"\nuchar\n");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar   ");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar ");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        PropertyBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_name() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"float \xc2\xae\n");
        let target = "\u{ae}".to_owned();
        let output = PropertyBlock::decode(source).unwrap_err();

        match output {
            Error::InvalidAscii(output) => assert_eq!(output, target),
            error => panic!("{error:?}"),
        }
    }

    #[test]
    fn default() {
        use super::*;

        let target = "default";
        let output = PropertyBlock::default().name;
        assert_eq!(output, target);

        let target =
            PropertyBlockVariant::Scalar(ScalarPropertyBlock::default());
        let output = PropertyBlockVariant::default();
        assert_eq!(output, target);
    }
}
