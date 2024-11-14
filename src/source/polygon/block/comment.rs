pub struct CommentBlock {
    pub message: String,
    pub variant: CommentVariant,
}

pub enum CommentVariant {
    Comment,
    ObjInfo,
}

impl CommentBlock {
    #[inline]
    pub const fn key(&self) -> &str {
        use CommentVariant::*;

        match self.variant {
            Comment => "comment",
            ObjInfo => "obj_info",
        }
    }
}
