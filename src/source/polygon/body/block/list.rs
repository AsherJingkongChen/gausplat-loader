pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyBlock {
    pub inner: Vec<Box<[u8]>>,
}

impl ListPropertyBlock {
    #[inline]
    pub fn cast_iter<T: Pod>(
        &self
    ) -> impl Iterator<Item = Result<&[T], Error>> {
        self.inner.iter().map(|bytes| Ok(try_cast_slice(bytes)?))
    }

    #[inline]
    pub fn cast_iter_mut<T: Pod>(
        &mut self
    ) -> impl Iterator<Item = Result<&mut [T], Error>> {
        self.inner
            .iter_mut()
            .map(|bytes| Ok(try_cast_slice_mut(bytes)?))
    }
}
