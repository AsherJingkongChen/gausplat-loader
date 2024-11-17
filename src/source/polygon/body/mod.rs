pub mod block;

pub use super::object::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use block::*;
pub use indexmap::IndexMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Body {
    pub blocks: IndexMap<Id, BodyBlock>,
}
