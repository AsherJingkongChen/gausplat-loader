pub use indexmap::IndexMap as IndexMapInner;
pub use rand::rngs::StdRng;
pub use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};

use indexmap::map::{rayon::IntoParIter, IntoIter};
use rand::{Rng, SeedableRng};
use std::{
    hash::{BuildHasher, Hash, RandomState},
    ops::{Deref, DerefMut},
};

/// A hash set where the iteration order of the values
/// is independent of their hash values.
///
/// This is an alias of [`IndexMap`] with empty values.
pub type IndexSet<T, S = RandomState> = IndexMap<T, (), S>;

/// A hash table where the iteration order of the key-value pairs
/// is independent of the hash values of the keys.
///
/// The interface is closely compatible with the standard [`HashMap`](std::collections::HashMap),
/// but also has additional features.
///
/// ## Order
///
/// See [`indexmap::IndexMap`] for more details.
///
/// ## Indices
///
/// See [`indexmap::IndexMap`] for more details.
///
/// ## Randomness
///
/// The [`IndexMap`] has a built-in random number generator ([`StdRng`]) for random operations.
///
/// It can be seeded with a [`u64`] value using the [`IndexMap::seed`] method.
///
/// ## Examples
///
/// ```rust
/// use gausplat_loader::collection::IndexMap;
///
/// let mut names = ["X", "X", "O"].into_iter().enumerate().collect::<IndexMap<_, _>>();
///
/// // Seed and sort values.
/// names.seed(212).sort_unstable_values();
///
/// // Get values in random order.
/// let names_target = vec!["X", "X", "X", "X", "O"];
/// let names_output = names.random_values().take(5).copied().collect::<Vec<_>>();
/// assert_eq!(names_output, names_target);
///
/// // Seed again.
/// names.seed(212);
///
/// // Get key-value pairs in random order.
/// let names_target = vec![(0, "X"), (1, "X"), (1, "X"), (1, "X"), (2, "O")];
/// let names_output = names.random_iter().take(5).map(|(&k, &v)| (k, v)).collect::<Vec<_>>();
/// assert_eq!(names_output, names_target);
/// ```
#[derive(Clone, Debug)]
pub struct IndexMap<K, V, S = RandomState> {
    inner: IndexMapInner<K, V, S>,
    rng: StdRng,
}

impl<K, V, S> IndexMap<K, V, S> {
    #[inline]
    pub fn into_inner(self) -> IndexMapInner<K, V, S> {
        self.inner
    }

    #[inline]
    pub fn seed(
        &mut self,
        seed: u64,
    ) -> &mut Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    /// Get a key-value pair by random.
    #[inline]
    pub fn get_random(&mut self) -> Option<(&K, &V)> {
        if self.inner.is_empty() {
            None?;
        }
        self.inner
            .get_index(self.rng.gen_range(0..self.inner.len()))
    }

    /// Get a key-value pair by random.
    #[inline]
    pub fn get_random_mut(&mut self) -> Option<(&K, &mut V)> {
        if self.inner.is_empty() {
            None?;
        }
        self.inner
            .get_index_mut(self.rng.gen_range(0..self.inner.len()))
    }

    /// Get a key by random.
    #[inline]
    pub fn get_random_key(&mut self) -> Option<&K> {
        self.get_random().map(|(key, _)| key)
    }

    /// Get a value by random.
    #[inline]
    pub fn get_random_value(&mut self) -> Option<&V> {
        self.get_random().map(|(_, value)| value)
    }

    /// Get a value by random.
    #[inline]
    pub fn get_random_value_mut(&mut self) -> Option<&mut V> {
        self.get_random_mut().map(|(_, value)| value)
    }

    /// Return an iterator over the key-value pairs of the map, in random order.
    #[inline]
    pub fn random_iter(&mut self) -> impl Iterator<Item = (&K, &V)> {
        (&mut self.rng)
            .sample_iter(rand::distributions::Uniform::new(0, self.inner.len()))
            .filter_map(|index| self.inner.get_index(index))
    }

    /// Return an iterator over the keys of the map, in random order.
    #[inline]
    pub fn random_keys(&mut self) -> impl Iterator<Item = &K> {
        self.random_iter().map(|(key, _)| key)
    }

    /// Return an iterator over the values of the map, in random order.
    #[inline]
    pub fn random_values(&mut self) -> impl Iterator<Item = &V> {
        self.random_iter().map(|(_, value)| value)
    }

    /// Shuffle the map’s key-value pairs in place.
    #[inline]
    pub fn shuffle(&mut self) -> &mut Self {
        for i in (1..self.inner.len()).rev() {
            self.inner.swap_indices(i, self.rng.gen_range(0..i));
        }
        self
    }

    /// Return an owning iterator over the keys of the map,
    /// in the order they were inserted.
    #[inline]
    pub fn into_keys(self) -> impl Iterator<Item = K> {
        self.into_iter().map(|(key, _)| key)
    }

    /// Return an owning iterator over the values of the map,
    /// in the order they were inserted.
    #[inline]
    pub fn into_values(self) -> impl Iterator<Item = V> {
        self.into_iter().map(|(_, value)| value)
    }
}

impl<K, V: Ord, S> IndexMap<K, V, S> {
    /// Sort the map’s key-value pairs by the default ordering of the values,
    /// and preserve the order of equal elements.
    ///
    /// Generally, [`sort_unstable_values`](IndexMap::sort_unstable_values)
    /// is faster than [`sort_values`](IndexMap::sort_values).
    ///
    /// See [`sort_by`](IndexMapInner::sort_by) for details.
    #[inline]
    pub fn sort_values(&mut self) -> &mut Self {
        self.inner.sort_by(|_, a, _, b| a.cmp(b));
        self
    }

    /// Sort the map's key-value pairs by the default ordering of the keys,
    /// but may not preserve the order of equal elements.
    ///
    /// See [`sort_unstable_by`](IndexMapInner::sort_unstable_by) for details.
    #[inline]
    pub fn sort_unstable_values(&mut self) -> &mut Self {
        self.inner.sort_unstable_by(|_, a, _, b| a.cmp(b));
        self
    }
}

impl<K, V> IndexMap<K, V> {
    /// Create a new map. (Does not allocate.)
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: IndexMapInner::new(),
            rng: StdRng::from_entropy(),
        }
    }

    /// Create a new map with capacity (Does not allocate if `capacity` is zero.)
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
    /// Return an empty [`IndexMap`] with a default RNG.
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
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
    /// Create an [`IndexMap`] from the sequence of key-value pairs in the iterable.
    ///
    /// `from_iter` uses the same logic as `extend`.
    /// See [`extend`][IndexMapInner::extend] for more details.
    #[inline]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        Self {
            inner: IndexMapInner::from_iter(iterable),
            rng: StdRng::from_entropy(),
        }
    }
}

impl<K, V, S> FromParallelIterator<(K, V)> for IndexMap<K, V, S>
where
    K: Hash + Eq + Send,
    V: Send,
    S: BuildHasher + Default + Send,
{
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = (K, V)>>(iterable: I) -> Self {
        Self {
            inner: IndexMapInner::from_par_iter(iterable),
            rng: StdRng::from_entropy(),
        }
    }
}

impl<K, V, S> IntoIterator for IndexMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K, V, S> IntoParallelIterator for IndexMap<K, V, S>
where
    K: Send,
    V: Send,
{
    type Iter = IntoParIter<K, V>;
    type Item = (K, V);

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.inner.into_par_iter()
    }
}

impl<K, V1, S1, V2, S2> PartialEq<IndexMap<K, V2, S2>> for IndexMap<K, V1, S1>
where
    K: Hash + Eq,
    V1: PartialEq<V2>,
    S1: BuildHasher,
    S2: BuildHasher,
{
    #[inline]
    fn eq(
        &self,
        other: &IndexMap<K, V2, S2>,
    ) -> bool {
        self.inner == other.inner
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_and_deref() {
        use super::*;

        let map = IndexMap::<u8, f32>::default();

        let target = true;
        let output = map.is_empty();
        assert_eq!(output, target);

        let output = (*map).is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn from_and_into_iter() {
        use super::*;

        let map =
            IndexMap::<u8, f32>::from_iter([(0, 0.1), (3, 0.4), (1, 0.2), (4, 0.5)]);

        let target = 4;
        let output = map.len();
        assert_eq!(output, target);

        let target = vec![0, 3, 1, 4];
        let output = map.to_owned().into_keys().collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = vec![0.1, 0.4, 0.2, 0.5];
        let output = map.to_owned().into_values().collect::<Vec<_>>();
        assert_eq!(output, target);
    }

    #[test]
    fn from_and_into_par_iter() {
        use super::*;

        let map =
            IndexMap::<u8, f32>::from_par_iter([(0, 0.1), (3, 0.4), (1, 0.2), (4, 0.5)]);

        let target = 4;
        let output = map.len();
        assert_eq!(output, target);

        let target = vec![0, 3, 1, 4];
        let output = map.to_owned().into_keys().collect::<Vec<_>>();
        assert_eq!(output, target);

        let target = vec![0.1, 0.4, 0.2, 0.5];
        let output = map.to_owned().into_values().collect::<Vec<_>>();
        assert_eq!(output, target);
    }

    #[test]
    fn get_random() {
        use super::*;

        let mut map = IndexMap::<u8, f32>::default();

        let target = None;
        let output = map.get_random_key();
        assert_eq!(output, target);

        let target = None;
        let output = map.get_random_value().copied();
        assert_eq!(output, target);

        let target = output;
        let output = map.get_random_value_mut().copied();
        assert_eq!(output, target);

        let mut map = IndexMap::<i16, f32>::with_capacity(1);
        map.insert(-123, -4.5);

        let target = Some(&-123);
        let output = map.get_random_key();
        assert_eq!(output, target);

        let target = Some(&-4.5);
        let output = map.get_random_value();
        assert_eq!(output, target);

        let target = Some(67.8);
        map.insert(-123, 67.8);
        let output = map.get_random_value_mut().copied();
        assert_eq!(output, target);
    }

    #[test]
    fn into_inner() {
        use super::*;

        let map = IndexMap::<u8, f32>::default();

        let target = map.is_empty();
        let output = map.into_inner().is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn new() {
        use super::*;

        let map = IndexMap::<u8, f32>::new();

        let target = true;
        let output = map.is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn seed_and_random_iter() {
        use super::*;

        let mut map: IndexMap<_, _> = [(0, 0.1), (3, 0.4), (1, 0.2), (4, 0.5)].into();

        let values_1 = map
            .seed(0)
            .random_keys()
            .copied()
            .take(256)
            .collect::<Vec<_>>();
        let values_2 = map
            .seed(0)
            .random_keys()
            .copied()
            .take(256)
            .collect::<Vec<_>>();
        assert_eq!(values_1, values_2);

        let values_1 = map
            .seed(0)
            .random_values()
            .copied()
            .take(256)
            .collect::<Vec<_>>();
        let values_2 = map
            .seed(0)
            .random_values()
            .copied()
            .take(256)
            .collect::<Vec<_>>();
        assert_eq!(values_1, values_2);
    }

    #[test]
    fn shuffle() {
        use super::*;

        let mut map = (0..=255).zip(0..=255).collect::<IndexMap<u8, u8>>();

        map.seed(0);

        let keys_1 = map.shuffle().keys().copied().collect::<Vec<_>>();
        let keys_2 = map.to_owned().shuffle().keys().copied().collect::<Vec<_>>();
        let values = map
            .to_owned()
            .shuffle()
            .values()
            .copied()
            .collect::<Vec<_>>();
        assert_ne!(keys_1, keys_2);
        assert_eq!(keys_2, values);
    }

    #[test]
    fn sort_values() {
        use super::*;

        let mut map = IndexMap::<u8, u32>::from_iter(
            [(0, 1), (2, 4), (1, 2), (3, 5)].iter().copied(),
        );

        map.sort_values();

        let target = vec![(0, 1), (1, 2), (2, 4), (3, 5)];
        let output = map.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>();
        assert_eq!(output, target);
    }

    #[test]
    fn sort_unstable_values() {
        use super::*;

        let mut map = IndexMap::<u8, u32>::from_iter(
            [(0, 1), (2, 4), (1, 2), (3, 5)].iter().copied(),
        );

        map.sort_unstable_values();

        let target = vec![(0, 1), (1, 2), (2, 4), (3, 5)];
        let output = map.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>();
        assert_eq!(output, target);
    }
}
