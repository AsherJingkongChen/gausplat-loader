pub use super::*;
pub use bytemuck::Pod;

use std::ops;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarData {
    inner: Vec<u8>,
}

impl ScalarData {
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn cast<T: Pod>(&self) -> Result<&[T], Error> {
        Ok(bytemuck::try_cast_slice(&self.inner)?)
    }

    #[inline]
    pub fn cast_mut<T: Pod>(&mut self) -> Result<&mut [T], Error> {
        Ok(bytemuck::try_cast_slice_mut(&mut self.inner)?)
    }
}

impl ops::Deref for ScalarData {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for ScalarData {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}