pub mod data;

pub use crate::error::Error;
pub use data::*;

use super::impl_variant_matchers;
use std::ops;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Body {
    inner: Vec<Vec<Data>>,
}

impl Body {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn iters(
        &self
    ) -> impl Iterator<Item = impl Iterator<Item = &Data>> {
        self.iter().map(|element| element.iter())
    }

    #[inline]
    pub fn iters_mut(
        &mut self
    ) -> impl Iterator<Item = impl Iterator<Item = &mut Data>> {
        self.iter_mut().map(|element| element.iter_mut())
    }
}

impl From<Vec<Vec<Data>>> for Body {
    #[inline]
    fn from(inner: Vec<Vec<Data>>) -> Self {
        Self { inner }
    }
}

impl ops::Deref for Body {
    type Target = Vec<Vec<Data>>;

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
