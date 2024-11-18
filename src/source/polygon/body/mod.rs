pub mod data;

pub use super::object::Id;
pub use crate::error::Error;
pub use data::*;
pub use indexmap::IndexMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Body {
    pub data_map: IndexMap<Id, Data>,
}
