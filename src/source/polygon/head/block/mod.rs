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

impl_head_block_variant_matchers!(Comment, Element, ObjInfo, Property);

macro_rules! impl_head_block_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl HeadBlockVariant {
                $(
                    #[inline]
                    pub fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake>](&self) -> Option<&[<$variant Block>]> {
                        match self {
                            Self::$variant(block) => Some(block),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_head_block_variant_matchers;
