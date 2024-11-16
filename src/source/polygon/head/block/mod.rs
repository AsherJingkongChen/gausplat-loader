pub mod comment;
pub mod element;
pub mod format;
pub mod property;

pub use super::Id;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};
pub use comment::*;
pub use element::*;
pub use format::*;
pub use property::*;

use crate::function::{
    is_space, read_byte_after, read_bytes_before, read_bytes_before_newline,
};
use std::io::Read;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HeadBlock {
    pub id: Id,
    pub variant: HeadBlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HeadBlockVariant {
    Format(FormatBlock),
    Element(ElementBlock),
    Property(PropertyBlock),
    Comment(CommentBlock),
    ObjInfo(ObjInfoBlock),
}
