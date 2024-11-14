pub mod list;
pub mod scalar;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};
pub use list::*;
pub use scalar::*;

use super::Id;
use crate::function::{read_byte_after, read_bytes_before_newline};
use std::io::Read;

/// ## Syntax
///
/// ```plaintext
/// <property-block> :=
///     | <property-variant> [{" "}] <name> ["\r"] "\n"
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyBlock {
    pub id: Id,
    pub name: AsciiString,
    pub variant: PropertyVariant,
}

/// ## Syntax
///
/// ```plaintext
/// <property-variant> :=
///     | [{" "}] "list" " " <list-property>
///     | <scalar-property>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyVariant {
    List(ListProperty),
    Scalar(ScalarProperty),
}

impl Decoder for PropertyBlock {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let variant = PropertyVariant::decode(reader)?;

        let mut name = vec![read_byte_after(reader, |b| b == b' ')?
            .ok_or_else(|| Error::MissingToken("<name>".into()))?];
        name.extend(read_bytes_before_newline(reader, 16)?);
        let name = name.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;

        Ok(Self {
            id: Default::default(),
            name,
            variant,
        })
    }
}

impl Decoder for PropertyVariant {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let list_or_scalar = ScalarProperty::decode(reader)?;
        Ok(match list_or_scalar.kind.as_bytes() {
            b"list" => Self::List(ListProperty::decode(reader)?),
            _ => Self::Scalar(list_or_scalar),
        })
    }
}

impl Default for PropertyVariant {
    #[inline]
    fn default() -> Self {
        Self::Scalar(Default::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_list() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"list uchar int vertex_indices\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "vertex_indices".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::List(ListProperty {
            count: UCHAR.to_owned(),
            entry: INT.to_owned(),
        });
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"list ushort uint    point_indices\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "point_indices".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::List(ListProperty {
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
        let target = "32x".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(FLOAT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"float32 x\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "x".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(FLOAT32.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "y".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y\n");
        let output = PropertyBlock::decode(source).unwrap();
        let target = "y".as_ascii_str().unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"int    y    \r\n");
        let target = "y    ".as_ascii_str().unwrap();
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"         int y\n");
        let target = "y".as_ascii_str().unwrap();
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(INT.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"uchar  \n");
        let target = "\n".as_ascii_str().unwrap();
        let output = PropertyBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = PropertyVariant::Scalar(UCHAR.to_owned());
        assert_eq!(output.variant, target);

        let source = &mut Cursor::new(b"\nuchar\n");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar   ");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar ");
        PropertyBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"uchar");
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

        let target = PropertyVariant::Scalar(ScalarProperty::default());
        let output = PropertyVariant::default();
        assert_eq!(output, target);
    }
}
