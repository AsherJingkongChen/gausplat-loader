//! Polygon payload module.

pub mod decode;

pub use super::*;

use derive_more::derive::{
    AsRef, Constructor, Deref, DerefMut, Display, From, IntoIterator, IsVariant,
    TryUnwrap,
};
use std::fmt;

/// Polygon payload variants.
#[derive(Clone, Debug, Display, Eq, Hash, From, IsVariant, PartialEq, TryUnwrap)]
#[try_unwrap(owned, ref, ref_mut)]
pub enum Payload {
    /// Scalar-only payload.
    Scalar(ScalarPayload),
    // TODO: Non-scalar payload
}

/// A payload that only contains scalar data.
#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Default,
    Deref,
    DerefMut,
    Eq,
    From,
    Hash,
    IntoIterator,
    PartialEq,
)]
pub struct ScalarPayload {
    /// Inner data list.
    pub data: Vec<Vec<Vec<u8>>>,
}

impl ScalarPayload {
    /// Returns the number of elements.
    #[inline]
    pub fn element_count(&self) -> usize {
        self.len()
    }

    /// Returns the number of properties.
    #[inline]
    pub fn property_count(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }

    /// Returns the number of bytes.
    #[inline]
    pub fn byte_count(&self) -> usize {
        self.iter()
            .map(|v| v.iter().map(Vec::len).sum::<usize>())
            .sum()
    }
}

impl Default for Payload {
    #[inline]
    fn default() -> Self {
        ScalarPayload::default().into()
    }
}

impl fmt::Display for ScalarPayload {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "[{} elements, {} properties, {} bytes]",
            self.element_count(),
            self.property_count(),
            self.byte_count()
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_and_display() {
        use super::*;

        let target = true;
        let output = Payload::default();
        assert_eq!(target, output.is_scalar());

        let mut output = output.try_unwrap_scalar().unwrap();
        output.data = vec![vec![vec![0u8; 4]; 7]; 3];
        output.data[0] = vec![vec![0u8; 4]; 8];
        let target = "[3 elements, 22 properties, 88 bytes]";
        let output = output.to_string();
        assert_eq!(target, output);
    }
}
