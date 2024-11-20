pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListData {
    inner: Vec<Box<[u8]>>,
}

impl ListData {
    #[inline]
    pub fn into_inner(self) -> Vec<Box<[u8]>> {
        self.inner
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn iter_cast<T: Pod>(&self) -> impl Iterator<Item = Result<&[T], Error>> {
        self.iter().map(|bytes| Ok(try_cast_slice(bytes)?))
    }

    #[inline]
    pub fn iter_cast_mut<T: Pod>(
        &mut self
    ) -> impl Iterator<Item = Result<&mut [T], Error>> {
        self.iter_mut().map(|bytes| Ok(try_cast_slice_mut(bytes)?))
    }
}

impl From<Vec<Box<[u8]>>> for ListData {
    #[inline]
    fn from(inner: Vec<Box<[u8]>>) -> Self {
        Self { inner }
    }
}

impl ops::Deref for ListData {
    type Target = Vec<Box<[u8]>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for ListData {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
