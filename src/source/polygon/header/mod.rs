pub mod decode;
pub mod encode;
pub mod property;

pub use super::*;
pub use indexmap::IndexMap;
pub use property::*;

use derive_more::derive::{
    AsRef, Constructor, Deref, DerefMut, Display, From, IntoIterator, IsVariant,
    TryUnwrap,
};
use std::{fmt, str::FromStr};
use Error::*;
use Format::*;

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
    IntoIterator,
    PartialEq,
)]
#[display("element {name} {count}\n{properties}")]
#[from((usize, String, Properties), (usize, &str, IndexMap<String, Property>))]
pub struct Element {
    pub count: usize,
    pub name: String,

    #[deref]
    #[deref_mut]
    #[into_iterator(owned, ref, ref_mut)]
    pub properties: Properties,
}

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
pub struct Elements {
    #[into_iterator(owned, ref, ref_mut)]
    pub inner: IndexMap<String, Element>,
}

#[derive(
    Clone, Copy, Debug, Default, Display, Eq, From, Hash, IsVariant, PartialEq, TryUnwrap,
)]
#[try_unwrap(owned, ref, ref_mut)]
pub enum Format {
    #[default]
    #[display("binary_little_endian")]
    BinaryLittleEndian,

    #[display("ascii")]
    Ascii,

    #[display("binary_big_endian")]
    BinaryBigEndian,
}

#[derive(
    AsRef,
    Clone,
    Constructor,
    Debug,
    Deref,
    DerefMut,
    Display,
    Eq,
    From,
    IntoIterator,
    PartialEq,
)]
#[display("ply\nformat {format} {version}\n{elements}end_header\n")]
#[from((Elements, Format, String), (IndexMap<String, Element>, Format, &str))]
pub struct Header {
    #[deref]
    #[deref_mut]
    #[into_iterator(owned, ref, ref_mut)]
    pub elements: Elements,
    pub format: Format,
    pub version: String,
}

impl Elements {
    #[inline]
    pub fn is_same_order(
        &self,
        other: &Self,
    ) -> bool {
        self.len().eq(&other.len())
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.0 == b.0 && a.1.is_same_order(b.1))
    }
}

impl Format {
    #[inline]
    pub const fn is_binary_native_endian(&self) -> bool {
        #[cfg(target_endian = "big")]
        return self.is_binary_big_endian();
        #[cfg(target_endian = "little")]
        return self.is_binary_little_endian();
    }

    #[inline]
    pub const fn binary_native_endian() -> Self {
        #[cfg(target_endian = "big")]
        return Self::BinaryBigEndian;
        #[cfg(target_endian = "little")]
        return Self::BinaryLittleEndian;
    }
}

impl Default for Header {
    #[inline]
    fn default() -> Self {
        Header {
            format: Default::default(),
            elements: Default::default(),
            version: "1.0".into(),
        }
    }
}

impl fmt::Display for Elements {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.values().try_for_each(|e| write!(f, "{e}"))
    }
}

impl FromStr for Header {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Header::decode(&mut std::io::Cursor::new(input))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn default() {
        use super::*;

        let taret = true;
        let output = &PropertyKind::default();
        assert_eq!(output.is_scalar(), taret);

        let target = true;
        let output = output.to_string().is_empty();
        assert_eq!(output, target);

        let target = "ply\nformat binary_little_endian 1.0\nend_header\n";
        let output = Header::default().to_string();
        assert_eq!(output, target);

        Header::decode(&mut Cursor::new(target)).unwrap();
    }

    #[test]
    fn is_same_order() {
        use super::*;

        let target = true;
        let output = Elements::default().is_same_order(&Elements::default());
        assert_eq!(output, target);

        let target = false;
        let mut elements = (
            "ply\n\
            format binary_little_endian 1.0\n\
            element vertex 365\n\
            property float x\n\
            property float y\n\
            end_header\n"
                .parse::<Header>()
                .unwrap()
                .elements,
            "ply\n\
            format binary_little_endian 1.0\n\
            element vertex 900\n\
            property float y\n\
            property float x\n\
            end_header\n"
                .parse::<Header>()
                .unwrap()
                .elements,
        );

        let output = elements.0.is_same_order(&elements.1);
        assert_eq!(output, target);

        let target = true;
        elements
            .1
            .get_mut("vertex")
            .unwrap()
            .properties
            .swap_indices(0, 1);
        let output = elements.0.is_same_order(&elements.1);
        assert_eq!(output, target);
    }

    #[test]
    fn format_on_native_endian() {
        use super::*;

        #[cfg(target_endian = "big")]
        {
            let target = Format::BinaryBigEndian;
            let output = Format::binary_native_endian();
            assert_eq!(output, target);
        }
        #[cfg(target_endian = "little")]
        {
            let target = Format::BinaryLittleEndian;
            let output = Format::binary_native_endian();
            assert_eq!(output, target);
        }
    }
}
