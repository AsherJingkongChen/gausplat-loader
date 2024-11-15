pub mod comment;
pub mod data;
pub mod element;
pub mod format;
pub mod property;

pub use super::group::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use ascii::{AsAsciiStr, AsciiStr, AsciiString, IntoAsciiString};
pub use comment::*;
pub use data::*;
pub use element::*;
pub use format::*;
pub use property::*;

pub struct Block {
    pub variant: BlockVariant,
}

pub enum BlockVariant {
    Ply,
    Format(FormatBlock),
    Element(ElementBlock),
    Property(PropertyBlock), // NOTE: Ok
    Comment(CommentBlock),   // NOTE: Ok
    ObjInfo(ObjInfoBlock),   // NOTE: Ok
    EndHeader,
    Data(DataBlock),
}

impl Block {
    #[inline]
    pub const fn key(&self) -> &str {
        use BlockVariant::*;

        match &self.variant {
            Property(_) => "property",
            Element(_) => "element",
            Data(_) => "",
            Ply => "ply",
            Format(_) => "format",
            EndHeader => "end_header",
            Comment(_) => "comment",
            ObjInfo(_) => "obj_info",
        }
    }
}
