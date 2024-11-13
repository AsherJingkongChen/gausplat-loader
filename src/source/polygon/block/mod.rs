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

pub enum BlockVariant {
    Ply,
    Format(FormatBlock),
    Element(ElementBlock),
    Property(PropertyBlock),
    Comment(CommentBlock),
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
            Comment(comment) => comment.key(),
        }
    }
}
