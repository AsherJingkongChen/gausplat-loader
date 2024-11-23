pub mod decode;
pub mod encode;

pub use super::*;
pub use indexmap::IndexMap;

use derive_more::derive::{Deref, DerefMut, Display, From};
use std::fmt;
use Error::*;
use Format::*;
use PropertyKind::*;

#[derive(Clone, Debug, Display, Eq, From, PartialEq)]
#[display("ply\nformat {format} {version}\n{elements}end_header\n")]
pub struct Header {
    pub elements: Elements,
    pub format: Format,
    pub version: String,
}

#[derive(Clone, Copy, Debug, Display, Default, Eq, From, Hash, PartialEq)]
pub enum Format {
    #[default]
    #[display("binary_little_endian")]
    BinaryLittleEndian,

    #[display("ascii")]
    Ascii,

    #[display("binary_big_endian")]
    BinaryBigEndian,
}

#[derive(Clone, Debug, Display, Default, Eq, From, PartialEq)]
#[display("element {name} {size}\n{properties}")]
pub struct Element {
    pub name: String,
    pub properties: Properties,
    pub size: usize,
}

#[derive(Clone, Debug, Display, Default, Eq, From, Hash, PartialEq)]
#[display("property {kind} {name}")]
pub struct Property {
    pub kind: PropertyKind,
    pub name: String,
}

#[derive(Clone, Debug, Display, Eq, Hash, From, PartialEq)]
pub enum PropertyKind {
    #[display("list {count} {value}")]
    List { count: String, value: String },

    #[display("{value}")]
    Scalar { value: String },
}

#[derive(Clone, Debug, Deref, DerefMut, Default, Eq, From, PartialEq)]
pub struct Elements(pub IndexMap<String, Element>);

#[derive(Clone, Debug, Deref, DerefMut, Default, Eq, From, PartialEq)]
pub struct Properties(pub IndexMap<String, Property>);

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
    }
}
