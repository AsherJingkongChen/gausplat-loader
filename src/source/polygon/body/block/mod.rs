pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyBlock {
    pub id: Id,
    pub variant: BodyBlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BodyBlockVariant {
    List(ListPropertyBlock),
    Scalar(ScalarPropertyBlock),
}
