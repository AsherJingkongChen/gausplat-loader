pub mod data;

pub use super::*;
pub use data::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BodyBlock {
    pub id: Id,
    pub data: IndexMap<Id, DataBlock>,
}
