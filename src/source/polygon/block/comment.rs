pub struct CommentBlock {
    pub message: String,
    pub variant: CommentBlockVariant,
}

pub enum CommentBlockVariant {
    Comment,
    ObjInfo,
}

impl CommentBlock {
    #[inline]
    pub const fn key(&self) -> &str {
        use CommentBlockVariant::*;

        match self.variant {
            Comment => "comment",
            ObjInfo => "obj_info",
        }
    }
}
