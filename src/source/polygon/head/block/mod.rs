pub mod comment;
pub mod element;
pub mod format;
pub mod property;

pub use super::*;

pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};
pub use comment::*;
pub use element::*;
pub use format::*;
pub use property::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HeadBlock {
    pub id: Id,
    pub variant: HeadBlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HeadBlockVariant {
    Element(ElementBlock),
    Property(PropertyBlock),
    Comment(CommentBlock),
    ObjInfo(ObjInfoBlock),
}
