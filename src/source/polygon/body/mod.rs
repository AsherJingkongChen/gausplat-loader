pub mod data;

pub use super::object::Id;
pub use crate::error::Error;
pub use data::*;
pub use indexmap::IndexMap;

use super::{impl_map_accessors, impl_variant_matchers};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Body {
    data_map: IndexMap<Id, Data>,
}

impl Body {
    impl_map_accessors!(Data, List, Scalar);

    #[inline]
    pub fn property_count(&self) -> usize {
        self.data_map.len()
    }
}

impl_body_data_accessors!(List, Scalar);
macro_rules! impl_body_data_accessors {
    ($( $data:ident ),* ) => {
        paste::paste! {
            impl Body {
                $(
                    #[inline]
                    pub fn [<get_or_init_ $data:snake _mut>](
                        &mut self,
                        id: &Id,
                        capacity: usize,
                    ) -> Option<&mut [<$data Data>]> {
                        use DataVariant::*;

                        let id = *id;
                        self.data_map
                            .entry(id)
                            .or_insert_with(|| Data {
                                id,
                                variant: [<$data>]([<$data Data>]::with_capacity(
                                    capacity,
                                )),
                            })
                            .variant
                            .[<as_ $data:snake _mut>]()
                    }
                )*
            }
        }
    };
}
use impl_body_data_accessors;
