pub mod data;

pub use data::*;
pub use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BodyBlock {
    pub id: Id,
    pub data: IndexMap<Id, DataBlock>,
}
