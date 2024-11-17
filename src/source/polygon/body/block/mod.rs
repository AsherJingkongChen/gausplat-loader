pub use super::*;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use indexmap::IndexMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BodyBlock {
    pub id: Id,
    pub properties: IndexMap<Id, Vec<u8>>,
}
