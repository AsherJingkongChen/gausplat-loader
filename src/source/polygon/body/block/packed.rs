pub use super::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackedBlock {
    pub inner: Vec<u8>,
}
