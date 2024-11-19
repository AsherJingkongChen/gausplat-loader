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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Meta {
    pub id: Id,
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
