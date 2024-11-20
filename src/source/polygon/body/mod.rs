pub mod data;

pub use crate::error::Error;
pub use data::*;

use super::impl_variant_matchers;
use std::ops;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Body {
    inner: Vec<ElementData>,
}

impl Body {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn iter_data(&self) -> impl Iterator<Item = &Data> {
        self.iter().flatten()
    }

    #[inline]
    pub fn iter_data_mut(&mut self) -> impl Iterator<Item = &mut Data> {
        self.iter_mut().flatten()
    }
}

impl From<Vec<ElementData>> for Body {
    #[inline]
    fn from(inner: Vec<ElementData>) -> Self {
        Self { inner }
    }
}

impl ops::Deref for Body {
    type Target = Vec<ElementData>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Body {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
