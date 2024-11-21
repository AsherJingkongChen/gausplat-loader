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
/// - [`ScalarPropertyBlockInfo`]
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyBlockInfo {
    pub count: ScalarPropertyBlockInfo,
    pub value: ScalarPropertyBlockInfo,
}

impl Decoder for ListPropertyBlockInfo {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let count = ScalarPropertyBlockInfo::decode(reader)?;
        let value = ScalarPropertyBlockInfo::decode(reader)?;
        Ok(Self { count, value })
    }
}

impl Encoder for ListPropertyBlockInfo {
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

impl fmt::Debug for ListPropertyBlockInfo {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:?} <> {:?}", self.count, self.value)
    }
}
