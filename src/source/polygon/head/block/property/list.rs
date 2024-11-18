pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <list-property-block> :=
///     | <scalar-property-block> <scalar-property-block>
/// ```
///
/// ### Syntax Reference
///
/// - [`ScalarPropertyBlock`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyBlock {
    pub count: ScalarPropertyBlock,
    pub value: ScalarPropertyBlock,
}

impl Decoder for ListPropertyBlock {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let count = ScalarPropertyBlock::decode(reader)?;
        let value = ScalarPropertyBlock::decode(reader)?;
        Ok(Self { count, value })
    }
}

impl Default for ListPropertyBlock {
    #[inline]
    fn default() -> Self {
        Self {
            count: UCHAR.to_owned(),
            value: INT.to_owned(),
        }
    }
}

impl Encoder for ListPropertyBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        self.count.encode(writer)?;
        self.value.encode(writer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default() {
        use super::*;

        let property = ListPropertyBlock::default();
        ScalarPropertyBlock::search(property.count.kind).unwrap();
        ScalarPropertyBlock::search(property.value.kind).unwrap();
    }
}
