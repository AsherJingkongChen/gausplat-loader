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
pub struct Meta {
    pub id: Id,
    pub variant: MetaVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MetaVariant {
    Element(ElementMeta),
    Property(PropertyMeta),
    Comment(CommentMeta),
    ObjInfo(ObjInfoMeta),
}

impl_head_meta_variant_matchers!(Comment, Element, ObjInfo, Property);

macro_rules! impl_head_meta_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl MetaVariant {
                $(
                    #[inline]
                    pub const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub const fn [<as_ $variant:snake>](&self) -> Option<&[<$variant Meta>]> {
                        match self {
                            Self::$variant(meta) => Some(meta),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_head_meta_variant_matchers;
