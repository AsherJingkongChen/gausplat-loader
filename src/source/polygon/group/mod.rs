pub mod id;

pub use id::*;
pub use indexmap::IndexMap;

pub struct Group {
    pub relations: IndexMap<Id, Vec<Id>>,
}
