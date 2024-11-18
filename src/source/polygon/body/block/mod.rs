pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub id: Id,
    pub variant: DataVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataVariant {
    List(ListData),
    Scalar(ScalarData),
}

impl_data_variant_matchers!(List, Scalar);

macro_rules! impl_data_variant_matchers {
    ($( $variant:ident ),* ) => {
        paste::paste! {
            impl DataVariant {
                $(
                    #[inline]
                    pub fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake>](&self) -> Option<&[<$variant Data>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }


                    #[inline]
                    pub fn [<as_ $variant:snake _mut>](&mut self) -> Option<&mut [<$variant Data>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}
use impl_data_variant_matchers;
