pub mod block;

pub use super::object::Id;
pub use crate::error::Error;
pub use block::*;
pub use indexmap::IndexMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Body {
    pub data_map: IndexMap<Id, Data>,
}
