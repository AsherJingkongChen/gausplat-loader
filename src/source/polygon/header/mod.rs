pub mod decode;

pub use super::*;
pub use indexmap::IndexMap;

use derive_more::derive::From;
use std::fmt;
use Error::*;
use Format::*;
use PropertyKind::*;

#[derive(Clone, Debug, Default, Eq, From, PartialEq)]
pub struct Header {
    pub elements: Elements,
    pub format: Format,
    pub version: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, From, Hash, PartialEq)]
pub enum Format {
    #[default]
    BinaryLittleEndian,
    Ascii,
    BinaryBigEndian,
}

#[derive(Clone, Debug, Default, Eq, From, PartialEq)]
pub struct Element {
    pub name: String,
    pub properties: Properties,
    pub size: usize,
}

#[derive(Clone, Debug, Default, Eq, From, Hash, PartialEq)]
pub struct Property {
    pub kind: PropertyKind,
    pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, From, PartialEq)]
pub enum PropertyKind {
    List { count: String, value: String },
    Scalar(String),
}

pub type Elements = IndexMap<String, Element>;
pub type Properties = IndexMap<String, Property>;

impl Default for PropertyKind {
    #[inline]
    fn default() -> Self {
        Scalar(Default::default())
    }
}

impl fmt::Display for Format {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            BinaryLittleEndian => write!(f, "binary_little_endian"),
            Ascii => write!(f, "ascii"),
            BinaryBigEndian => write!(f, "binary_big_endian"),
        }
    }
}

#[cfg(test)]
#[test]
fn default() {
    use super::*;

    let kind_target = &Scalar(Default::default());
    let kind_output = &PropertyKind::default();
    assert_eq!(kind_output, kind_target);
}
