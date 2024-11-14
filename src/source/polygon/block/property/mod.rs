pub mod list;
pub mod scalar;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use list::*;
pub use scalar::*;

/// ## Syntax
///
/// ```plaintext
/// <property-block> :=
///     | <property-variant> <name>
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyBlock {
    pub id: u32,
    pub name: String,
    pub variant: PropertyVariant,
}

/// ## Syntax
///
/// ```plaintext
/// <property-variant> :=
///     | list <list-property>
///     | <scalar-property>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyVariant {
    List(ListProperty),
    Scalar(ScalarProperty),
}

impl Default for PropertyVariant {
    #[inline]
    fn default() -> Self {
        Self::Scalar(Default::default())
    }
}
