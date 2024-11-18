pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyBlock {
    pub id: Id,
    pub variant: BodyBlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BodyBlockVariant {
    List(ListBodyBlock),
    Scalar(ScalarBodyBlock),
}

impl_body_block_variant_matchers!(List, Scalar);

macro_rules! impl_body_block_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl BodyBlockVariant {
                $(
                    #[inline]
                    pub fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake>](&self) -> Option<&[<$variant BodyBlock>]> {
                        match self {
                            Self::$variant(block) => Some(block),
                            _ => None,
                        }
                    }


                    #[inline]
                    pub fn [<as_ $variant:snake _mut>](&mut self) -> Option<&mut [<$variant BodyBlock>]> {
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
use impl_body_block_variant_matchers;
