pub mod comment;
pub mod element;
pub mod format;
pub mod property;

pub use super::*;
pub use ascii::{AsciiString, IntoAsciiString};
pub use comment::*;
pub use element::*;
pub use format::*;
pub use property::*;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Meta {
    pub variant: MetaVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MetaVariant {
    Comment(CommentMeta),
    Element(ElementMeta),
    ObjInfo(ObjInfoMeta),
    Property(PropertyMeta),
}

impl_variant_matchers!(Meta, Comment, Element, ObjInfo, Property);

impl Default for MetaVariant {
    #[inline]
    fn default() -> Self {
        Self::Comment(Default::default())
    }
}

impl From<MetaVariant> for Meta {
    #[inline]
    fn from(variant: MetaVariant) -> Self {
        Self { variant }
    }
}

impl ops::Deref for Meta {
    type Target = MetaVariant;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl ops::DerefMut for Meta {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}
