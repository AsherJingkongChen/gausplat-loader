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
use std::fmt;
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
pub struct Header {
    #[deref]
    #[deref_mut]
    #[into_iterator(owned, ref, ref_mut)]
    pub elements: Elements,
    pub format: Format,
    pub version: String,
}

impl Format {
    #[inline]
    pub const fn is_binary_native_endian(&self) -> bool {
        match self {
            BinaryLittleEndian => cfg!(target_endian = "little"),
            BinaryBigEndian => cfg!(target_endian = "big"),
            Ascii => false,
        }
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
        self.values().try_for_each(|e| write!(f, "{e}\n"))
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
}
