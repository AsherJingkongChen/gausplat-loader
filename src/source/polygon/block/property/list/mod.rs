pub mod data;
pub mod info;

pub use super::*;
pub use data::*;
pub use info::*;

use std::fmt;

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyBlock {
    pub data: ListPropertyBlockData,
    pub info: ListPropertyBlockInfo,
}

impl fmt::Debug for ListPropertyBlock {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{{ {:?} x{:?} }}", self.info, self.data.len())
    }
}

impl ops::Deref for ListPropertyBlock {
    type Target = ListPropertyBlockInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl ops::DerefMut for ListPropertyBlock {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}
