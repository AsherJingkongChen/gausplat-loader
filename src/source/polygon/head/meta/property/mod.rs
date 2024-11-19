pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

/// ## Syntax
///
/// ```plaintext
/// <property-meta> :=
///     | <property-meta-variant> [{" "}] <name> <newline>
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
/// - [`PropertyMetaVariant`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PropertyMeta {
    pub name: AsciiString,
    pub variant: PropertyMetaVariant,
}

/// ## Syntax
///
/// ```plaintext
/// <property-meta-variant> :=
///     | [{" "}] "list" " " <list-property-meta>
///     | <scalar-property-meta>
/// ```
///
/// ### Syntax Reference
///
/// - [`ListPropertyMeta`]
/// - [`ScalarPropertyMeta`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PropertyMetaVariant {
    List(ListPropertyMeta),
    Scalar(ScalarPropertyMeta),
}

impl_property_meta_variant_matchers!(List, Scalar);

impl Decoder for PropertyMeta {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let variant = PropertyMetaVariant::decode(reader)?;

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

impl Decoder for PropertyMetaVariant {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let list_or_scalar = ScalarPropertyMeta::decode(reader)?;
        Ok(match list_or_scalar.kind.as_bytes() {
            b"list" => Self::List(ListPropertyMeta::decode(reader)?),
            _ => Self::Scalar(list_or_scalar),
        })
    }
}

impl Default for PropertyMeta {
    #[inline]
    fn default() -> Self {
        let name = "default".into_ascii_string().expect("Unreachable");
        let variant = Default::default();
        Self { name, variant }
    }
}

impl Default for PropertyMetaVariant {
    #[inline]
    fn default() -> Self {
        Self::Scalar(Default::default())
    }
}

impl Encoder for PropertyMeta {
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

impl Encoder for PropertyMetaVariant {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        match self {
            Self::Scalar(scalar) => scalar.encode(writer),
            Self::List(list) => {
                write_bytes(writer, b"list ")?;
                list.encode(writer)
            },
        }
    }
}

macro_rules! impl_property_meta_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl PropertyMetaVariant {
                $(
                    #[inline]
                    pub fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake>](&self) -> Option<&[<$variant PropertyMeta>]> {
                        match self {
                            Self::$variant(meta) => Some(meta),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_property_meta_variant_matchers;

#[cfg(test)]
mod tests {
    #[test]
    fn decode_list() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"list uchar int vertex_indices\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "vertex_indices";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::List(ListPropertyMeta {
            count: UCHAR.to_owned(),
            value: INT.to_owned(),
        });
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"list ushort uint    point_indices\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "point_indices";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::List(ListPropertyMeta {
            count: USHORT.to_owned(),
            value: UINT.to_owned(),
        });
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"listuchar int vertex_indices\n");
        PropertyMeta::decode(source).unwrap_err();
    }

    #[test]
    fn decode_scalar() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"float 32x\r\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "32x";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(FLOAT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"float32 x\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "x";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(FLOAT32.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "y";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyMeta::decode(source).unwrap();
        let target = "y";
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y    \r\n");
        let target = "y    ";
        let output = PropertyMeta::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"         int y\n");
        let target = "y";
        let output = PropertyMeta::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"uchar  \n");
        let target = "\n";
        let output = PropertyMeta::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyMetaVariant::Scalar(UCHAR.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"\nuchar\n");
        PropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar   ");
        PropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar ");
        PropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar");
        PropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        PropertyMeta::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_name() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"float \xc2\xae\n");
        let target = "\u{ae}".to_owned();
        let output = PropertyMeta::decode(source).unwrap_err();

        match output {
            Error::InvalidAscii(output) => assert_eq!(output, target),
            error => panic!("{error:?}"),
        }
    }

    #[test]
    fn default() {
        use super::*;

        let target = "default";
        let output = PropertyMeta::default().name;
        assert_eq!(output, target);

        let target = PropertyMetaVariant::Scalar(ScalarPropertyMeta::default());
        let output = PropertyMetaVariant::default();
        assert_eq!(output, target);
    }

    #[test]
    fn matcher_is() {
        use super::*;

        let target = true;
        let output =
            PropertyMetaVariant::List(ListPropertyMeta::default()).is_list();
        assert_eq!(output, target);

        let target = true;
        let output = PropertyMetaVariant::Scalar(ScalarPropertyMeta::default())
            .is_scalar();
        assert_eq!(output, target);

        let target = false;
        let output = PropertyMetaVariant::Scalar(ScalarPropertyMeta::default())
            .is_list();
        assert_eq!(output, target);

        let target = false;
        let output =
            PropertyMetaVariant::List(ListPropertyMeta::default()).is_scalar();
        assert_eq!(output, target);
    }

    #[test]
    fn matcher_as() {
        use super::*;

        let target = Some(ListPropertyMeta::default());
        let output = PropertyMetaVariant::List(ListPropertyMeta::default())
            .as_list()
            .cloned();
        assert_eq!(output, target);

        let target = Some(ScalarPropertyMeta::default());
        let output = PropertyMetaVariant::Scalar(ScalarPropertyMeta::default())
            .as_scalar()
            .cloned();
        assert_eq!(output, target);

        let target = None;
        let output = PropertyMetaVariant::Scalar(ScalarPropertyMeta::default())
            .as_list()
            .cloned();
        assert_eq!(output, target);

        let target = None;
        let output = PropertyMetaVariant::List(ListPropertyMeta::default())
            .as_scalar()
            .cloned();
        assert_eq!(output, target);
    }
}
