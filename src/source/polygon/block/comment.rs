pub struct CommentBlock {
    pub message: String,
    pub variant: CommentBlockVariant,
}

pub enum CommentBlockVariant {
    Comment,
    ObjInfo,
}
