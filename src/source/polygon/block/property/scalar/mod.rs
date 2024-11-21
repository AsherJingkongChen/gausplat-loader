pub mod data;
pub mod info;

pub use super::*;
pub use data::*;
pub use info::*;

use std::fmt;

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarPropertyBlock {
    pub data: ScalarPropertyBlockData,
    pub info: ScalarPropertyBlockInfo,
}

impl fmt::Debug for ScalarPropertyBlock {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{{ {:?} x{:?} }}", self.info, self.data.len())
    }
}

impl ops::Deref for ScalarPropertyBlock {
    type Target = ScalarPropertyBlockInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl ops::DerefMut for ScalarPropertyBlock {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}
