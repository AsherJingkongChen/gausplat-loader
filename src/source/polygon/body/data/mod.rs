pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub id: Id,
    pub variant: DataVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataVariant {
    List(ListData),
    Scalar(ScalarData),
}

impl_variant_matchers!(Data, List, Scalar);
