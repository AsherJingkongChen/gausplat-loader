pub mod comment;
pub mod data;
pub mod element;
pub mod format;
pub mod property;

pub use comment::*;
pub use data::*;
pub use element::*;
pub use format::*;
pub use property::*;

pub struct Block {
    pub variant: BlockVariant,
}

impl Block {
    #[inline]
    pub const fn key(&self) -> &str {
        match &self.variant {
            BlockVariant::Ply => "ply",
            BlockVariant::Format(_) => "format",
            BlockVariant::Element(_) => "element",
            BlockVariant::Property(_) => "property",
            BlockVariant::Comment(_) => "comment",
            BlockVariant::EndHeader => "end_header",
            BlockVariant::Data(_) => "",
        }
    }
}

pub enum BlockVariant {
    Ply,
    Format(FormatBlock),
    Element(ElementBlock),
    Property(PropertyBlock),
    Comment(CommentBlock),
    EndHeader,
    Data(DataBlock),
}
