pub mod comment;
pub mod element;
pub mod property;

pub use super::*;
pub use ascii::{AsciiString, IntoAsciiString};
pub use comment::*;
pub use element::*;
pub use property::*;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Block {
    pub variant: BlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BlockVariant {
    Comment(CommentBlock),
    Element(ElementBlock),
    ObjInfo(ObjInfoBlock),
    Property(PropertyBlock),
}

impl_variant_matchers! { Block, Comment, Element, ObjInfo, Property }

impl Default for BlockVariant {
    #[inline]
    fn default() -> Self {
        Self::Comment(Default::default())
    }
}
