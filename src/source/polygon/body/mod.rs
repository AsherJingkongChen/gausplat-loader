pub mod data;

pub use super::object::Id;
pub use crate::error::Error;
pub use data::*;
pub use indexmap::IndexMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Body {
    pub data_map: IndexMap<Id, Data>,
}

impl Body {
    #[inline]
    pub fn get_list(
        &self,
        id: Id,
    ) -> Option<&ListData> {
        self.data_map.get(&id).and_then(|d| d.variant.as_list())
    }

    #[inline]
    pub fn get_scalar(
        &self,
        id: Id,
    ) -> Option<&ScalarData> {
        self.data_map.get(&id).and_then(|d| d.variant.as_scalar())
    }

    #[inline]
    pub fn get_list_mut(
        &mut self,
        id: Id,
        capacity: usize,
    ) -> Option<&mut ListData> {
        self.data_map
            .entry(id)
            .or_insert_with(|| Data {
                id,
                variant: DataVariant::List(ListData::with_capacity(capacity)),
            })
            .variant
            .as_list_mut()
    }

    #[inline]
    pub fn get_scalar_mut(
        &mut self,
        id: Id,
        capacity: usize,
    ) -> Option<&mut ScalarData> {
        self.data_map
            .entry(id)
            .or_insert_with(|| Data {
                id,
                variant: DataVariant::Scalar(ScalarData::with_capacity(
                    capacity,
                )),
            })
            .variant
            .as_scalar_mut()
    }
}
