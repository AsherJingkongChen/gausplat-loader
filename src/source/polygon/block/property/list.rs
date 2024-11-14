pub use super::*;

use std::io::Read;

/// ## Syntax
///
/// ```plaintext
/// <list-property> :=
///     | <scalar-property> <scalar-property>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ListProperty {
    pub count: ScalarProperty,
    pub entry: ScalarProperty,
}

impl Decoder for ListProperty {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let count = ScalarProperty::decode(reader)?;
        let entry = ScalarProperty::decode(reader)?;
        Ok(Self { count, entry })
    }
}

impl Default for ListProperty {
    #[inline]
    fn default() -> Self {
        Self {
            count: UCHAR.to_owned(),
            entry: INT.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;

        let source = &mut std::io::Cursor::new(b"uchar int ");
        let target = ListProperty {
            count: UCHAR.to_owned(),
            entry: INT.to_owned(),
        };
        let output = ListProperty::decode(source).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn default() {
        use super::*;

        let property = ListProperty::default();
        ScalarProperty::search(property.count.kind.as_slice()).unwrap();
        ScalarProperty::search(property.entry.kind.as_slice()).unwrap();
    }
}
