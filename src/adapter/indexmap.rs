pub use indexmap::IndexMap as IndexMapInner;
pub use rand::rngs::StdRng;

use rand::{Rng, SeedableRng};
use std::{
    hash::{BuildHasher, Hash, RandomState},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug)]
pub struct IndexMap<K, V, S = RandomState> {
    pub inner: IndexMapInner<K, V, S>,
    pub rng: StdRng,
}

impl<K, V, S> IndexMap<K, V, S> {
    #[inline]
    pub fn seed(
        &mut self,
        seed: u64,
    ) -> &mut Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    pub fn get_random(&mut self) -> Option<(&K, &V)> {
        self.inner
            .get_index(self.rng.gen_range(0..self.inner.len()))
    }

    pub fn get_random_mut(&mut self) -> Option<(&K, &mut V)> {
        self.inner
            .get_index_mut(self.rng.gen_range(0..self.inner.len()))
    }

    #[inline]
    pub fn get_random_key(&mut self) -> Option<&K> {
        self.get_random().map(|(key, _)| key)
    }

    #[inline]
    pub fn get_random_value(&mut self) -> Option<&V> {
        self.get_random().map(|(_, value)| value)
    }

    #[inline]
    pub fn get_random_value_mut(&mut self) -> Option<&mut V> {
        self.get_random_mut().map(|(_, value)| value)
    }

    pub fn random_iter(&mut self) -> impl Iterator<Item = (&K, &V)> {
        (&mut self.rng)
            .sample_iter(rand::distributions::Uniform::new(0, self.inner.len()))
            .filter_map(|index| self.inner.get_index(index))
    }

    #[inline]
    pub fn random_keys(&mut self) -> impl Iterator<Item = &K> {
        self.random_iter().map(|(key, _)| key)
    }

    #[inline]
    pub fn random_values(&mut self) -> impl Iterator<Item = &V> {
        self.random_iter().map(|(_, value)| value)
    }
}

impl<K, V> IndexMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: IndexMapInner::new(),
            rng: StdRng::from_entropy(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMapInner::with_capacity(capacity),
            rng: StdRng::from_entropy(),
        }
    }
}

impl<K, V, S> Default for IndexMap<K, V, S>
where
    S: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: IndexMapInner::default(),
            rng: StdRng::from_seed(Default::default()),
        }
    }
}

impl<K, V, S> Deref for IndexMap<K, V, S> {
    type Target = IndexMapInner<K, V, S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V, S> DerefMut for IndexMap<K, V, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for IndexMap<K, V>
where
    K: Hash + Eq,
{
    #[inline]
    fn from(value: [(K, V); N]) -> Self {
        Self {
            inner: IndexMapInner::from(value),
            rng: StdRng::from_entropy(),
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for IndexMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher + Default,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        Self {
            inner: IndexMapInner::from_iter(iterable),
            rng: StdRng::from_entropy(),
        }
    }
}
