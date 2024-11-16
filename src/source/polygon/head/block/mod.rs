pub mod comment;
pub mod element;
pub mod format;
pub mod property;

pub use super::*;
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

pub struct HeadBlock {
    pub variant: HeadBlockVariant,
}

pub enum HeadBlockVariant {
    Ply,
    Format(FormatBlock), // NOTE: Ok
    Element(ElementBlock), // NOTE: Ok
    Property(PropertyBlock), // NOTE: Ok
    Comment(CommentBlock),   // NOTE: Ok
    ObjInfo(ObjInfoBlock),   // NOTE: Ok
    EndHeader,
}
