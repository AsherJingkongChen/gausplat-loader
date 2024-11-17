pub use super::*;
pub use bytemuck::Pod;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarPropertyBlock {
    pub inner: Vec<u8>,
}

impl ScalarPropertyBlock {
    #[inline]
    pub fn cast<T: Pod>(&self) -> Result<&[T], Error> {
        Ok(bytemuck::try_cast_slice(&self.inner)?)
    }

    #[inline]
    pub fn cast_mut<T: Pod>(&mut self) -> Result<&mut [T], Error> {
        Ok(bytemuck::try_cast_slice_mut(&mut self.inner)?)
    }
}
