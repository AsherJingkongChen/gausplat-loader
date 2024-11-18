pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <list-property-meta> :=
///     | <scalar-property-meta> <scalar-property-meta>
/// ```
///
/// ### Syntax Reference
///
/// - [`ScalarPropertyMeta`]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyMeta {
    pub count: ScalarPropertyMeta,
    pub value: ScalarPropertyMeta,
}

impl Decoder for ListPropertyMeta {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let count = ScalarPropertyMeta::decode(reader)?;
        let value = ScalarPropertyMeta::decode(reader)?;
        Ok(Self { count, value })
    }
}

impl Default for ListPropertyMeta {
    #[inline]
    fn default() -> Self {
        Self {
            count: UCHAR.to_owned(),
            value: INT.to_owned(),
        }
    }
}

impl Encoder for ListPropertyMeta {
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

        let property = ListPropertyMeta::default();
        ScalarPropertyMeta::search(property.count.kind).unwrap();
        ScalarPropertyMeta::search(property.value.kind).unwrap();
    }
}
