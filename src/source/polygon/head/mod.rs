pub mod block;

pub use super::group::Id;
pub use block::*;
pub use indexmap::IndexMap;

pub struct Head {
    pub blocks: IndexMap<Id, HeadBlock>,
}
