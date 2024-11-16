pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <list-property> :=
///     | <scalar-property> <scalar-property>
/// ```
///
/// ### Syntax Reference
///
/// - [`ScalarProperty`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl Encoder for ListProperty {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        self.count.encode(writer)?;
        self.entry.encode(writer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default() {
        use super::*;

        let property = ListProperty::default();
        ScalarProperty::search(property.count.kind).unwrap();
        ScalarProperty::search(property.entry.kind).unwrap();
    }
}
