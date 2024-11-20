pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Data {
    variant: DataVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataVariant {
    List(ListData),
    Scalar(ScalarData),
}

impl_variant_matchers!(Data, List, Scalar);

impl Default for DataVariant {
    #[inline]
    fn default() -> Self {
        Self::Scalar(Default::default())
    }
}

impl From<DataVariant> for Data {
    #[inline]
    fn from(variant: DataVariant) -> Self {
        Self { variant }
    }
}

impl ops::Deref for Data {
    type Target = DataVariant;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl ops::DerefMut for Data {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}
