pub use super::*;

use std::fmt;

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
#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl fmt::Debug for ListPropertyMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "list<{:?}, {:?}>", self.count, self.value)
    }
}
