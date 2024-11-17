pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataBlock {
    pub id: Id,
    pub inner: Vec<u8>,
}

impl DataBlock {
    #[inline]
    pub fn as_scalars<T: Pod>(&self) -> Result<&[T], Error> {
        Ok(try_cast_slice(&self.inner)?)
    }

    #[inline]
    pub fn as_scalars_mut<T: Pod>(&mut self) -> Result<&mut [T], Error> {
        Ok(try_cast_slice_mut(&mut self.inner)?)
    }

    #[inline]
    pub fn as_lists<T: Pod>(&self) -> Result<&[T], Error> {
        unimplemented!()
    }
}
