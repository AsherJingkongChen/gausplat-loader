pub mod decode;
pub mod encode;

pub use super::*;
pub use indexmap::IndexMap;

use derive_more::derive::{Deref, DerefMut, Display, From, IntoIterator, IsVariant};
use std::fmt;
use Error::*;
use Format::*;
use PropertyKind::*;

#[derive(
    Clone, Debug, Default, Deref, DerefMut, Display, Eq, From, IntoIterator, PartialEq,
)]
#[display("element {name} {size}\n{properties}")]
pub struct Element {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[into_iterator(owned, ref, ref_mut)]
    pub properties: Properties,
    pub size: usize,
}

#[derive(Clone, Debug, Deref, DerefMut, Default, Eq, From, IntoIterator, PartialEq)]
pub struct Elements {
    #[into_iterator(owned, ref, ref_mut)]
    inner: IndexMap<String, Element>,
}

#[derive(Clone, Copy, Debug, Default, Display, Eq, From, Hash, IsVariant, PartialEq)]
pub enum Format {
    #[default]
    #[display("binary_little_endian")]
    BinaryLittleEndian,

    #[display("ascii")]
    Ascii,

    #[display("binary_big_endian")]
    BinaryBigEndian,
}

#[derive(Clone, Debug, Deref, DerefMut, Display, Eq, From, IntoIterator, PartialEq)]
#[display("ply\nformat {format} {version}\n{elements}end_header\n")]
pub struct Header {
    #[deref]
    #[deref_mut]
    #[into_iterator(owned, ref, ref_mut)]
    pub elements: Elements,
    pub format: Format,
    pub version: String,
}

#[derive(Clone, Debug, Default, Display, Eq, From, Hash, PartialEq)]
#[display("property {kind} {name}")]
pub struct Property {
    pub kind: PropertyKind,
    pub name: String,
}

#[derive(Clone, Debug, Display, Eq, Hash, From, IsVariant, PartialEq)]
pub enum PropertyKind {
    #[display("list {count} {value}")]
    List { count: String, value: String },

    #[display("{value}")]
    Scalar { value: String },
}

#[derive(Clone, Debug, Deref, DerefMut, Default, Eq, From, IntoIterator, PartialEq)]
pub struct Properties {
    #[into_iterator(owned, ref, ref_mut)]
    inner: IndexMap<String, Property>,
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
        self.values().try_for_each(|e| write!(f, "{e}\n"))
    }
}

impl fmt::Display for Properties {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let last_index = self.len().saturating_sub(1);
        self.values().enumerate().try_for_each(|(index, property)| {
            let newline = if index == last_index { "" } else { "\n" };
            write!(f, "{property}{newline}")
        })
    }
}

impl Default for PropertyKind {
    #[inline]
    fn default() -> Self {
        Scalar {
            value: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn default() {
        use super::*;

        let kind_target = &Default::default();
        let kind_output = &PropertyKind::default();
        assert_eq!(kind_output, kind_target);

        let target = true;
        let output = kind_output.to_string().is_empty();
        assert_eq!(output, target);

        let target = "ply\nformat binary_little_endian 1.0\nend_header\n";
        let output = Header::default().to_string();
        assert_eq!(output, target);

        Header::decode(&mut Cursor::new(target)).unwrap();
    }
}
