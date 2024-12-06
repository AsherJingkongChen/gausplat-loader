//! Polygon header property.

pub use super::*;

use std::sync::{LazyLock, RwLock};

/// List property kind.
#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Default,
    Deref,
    DerefMut,
    Display,
    Eq,
    Hash,
    From,
    PartialEq,
)]
#[display("list {count} {value}")]
#[from((String, String), (&str, &str))]
pub struct ListPropertyKind {
    /// The property kind of the list count.
    pub count: ScalarPropertyKind,

    /// The property kind of the list value.
    #[as_ref]
    #[deref]
    #[deref_mut]
    pub value: ScalarPropertyKind,
}

/// Polygon Property.
#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Default,
    Deref,
    DerefMut,
    Display,
    Eq,
    From,
    Hash,
    PartialEq,
)]
#[display("property {kind} {name}")]
#[from((PropertyKind, String), (PropertyKind, &str))]
pub struct Property {
    /// Property kind.
    #[deref]
    #[deref_mut]
    pub kind: PropertyKind,
    /// Property name.
    pub name: String,
}

/// Property kind variants.
#[derive(Clone, Debug, Display, Eq, Hash, From, IsVariant, PartialEq, TryUnwrap)]
#[try_unwrap(owned, ref, ref_mut)]
pub enum PropertyKind {
    /// List property kind.
    List(ListPropertyKind),
    /// Scalar property kind.
    Scalar(ScalarPropertyKind),
}

/// A map of [`Property::name`] to [`Property`].
#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Deref,
    DerefMut,
    Default,
    Eq,
    From,
    IntoIterator,
    PartialEq,
)]
pub struct Properties {
    /// Inner map.
    #[into_iterator(owned, ref, ref_mut)]
    pub inner: IndexMap<String, Property>,
}

/// Scalar property kind.
#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Default,
    Deref,
    DerefMut,
    Display,
    Eq,
    Hash,
    From,
    PartialEq,
)]
#[display("{value}")]
#[from(String, &str)]
pub struct ScalarPropertyKind {
    /// Kind representation.
    pub value: String,
}

impl Properties {
    /// Check if the two properties have the same order.
    ///
    /// It can be used for checking the compatibility of two polygon objects.
    #[inline]
    pub fn is_same_order(
        &self,
        other: &Self,
    ) -> bool {
        self.len().eq(&other.len()) && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }

    /// Return an iterator of property sizes.
    #[inline]
    pub fn property_sizes(&self) -> impl '_ + Iterator<Item = Result<usize, Error>> {
        self.values().map(|prop| {
            prop.try_unwrap_scalar_ref()
                .map_err(|err| InvalidKind(err.input.to_string()))?
                .size()
                .ok_or_else(|| InvalidKind(prop.kind.to_string()))
        })
    }
}

impl ScalarPropertyKind {
    /// Query the size of the scalar property.
    #[inline]
    pub fn size(&self) -> Option<usize> {
        SCALAR_PROPERTY_SIZES
            .read()
            .unwrap()
            .get(&self.value)
            .copied()
    }
}

impl Default for PropertyKind {
    #[inline]
    fn default() -> Self {
        ScalarPropertyKind::default().into()
    }
}

impl fmt::Display for Properties {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.values().try_for_each(|p| writeln!(f, "{p}"))
    }
}

/// A map of scalar property kind to its size.
pub static SCALAR_PROPERTY_SIZES: LazyLock<RwLock<IndexMap<String, usize>>> =
    LazyLock::new(|| {
        [
            // Common scalar types
            ("char".into(), 1),
            ("uchar".into(), 1),
            ("short".into(), 2),
            ("ushort".into(), 2),
            ("int".into(), 4),
            ("uint".into(), 4),
            ("float".into(), 4),
            ("double".into(), 8),
            // General scalar types
            ("int8".into(), 1),
            ("uint8".into(), 1),
            ("int16".into(), 2),
            ("uint16".into(), 2),
            ("int32".into(), 4),
            ("uint32".into(), 4),
            ("float32".into(), 4),
            ("float64".into(), 8),
            // Special scalar types
            ("byte".into(), 1),
            ("ubyte".into(), 1),
            ("half".into(), 2),
            ("long".into(), 8),
            ("ulong".into(), 8),
            ("float16".into(), 2),
            ("int64".into(), 8),
            ("uint64".into(), 8),
        ]
        .into_iter()
        .collect::<IndexMap<_, _>>()
        .into()
    });
