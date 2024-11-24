pub mod decode;
pub mod encode;

pub use super::*;
pub use bytemuck::Pod;
pub use header::{Element, Elements, Properties, Property};

use bytemuck::try_cast_slice;
use derive_more::derive::{AsRef, Constructor, Display, From};
use Error::*;

#[derive(AsRef, Clone, Constructor, Debug, Display, Eq, From, PartialEq)]
#[display("{header}----------\n{payload}")]
pub struct Object {
    pub header: Header,
    pub payload: Payload,
}

// TODO: Implement mutable accessors for `Object`.

impl Object {
    #[doc(alias = "properties")]
    #[inline]
    pub fn get_element(
        &self,
        element_name: &str,
    ) -> Option<(&Element, &Vec<Vec<u8>>)> {
        let (index, _, element) = self.header.get_full(element_name)?;
        Some((
            element,
            self.payload
                .try_unwrap_scalar_ref()
                .unwrap()
                .data
                .get(index)?,
        ))
    }

    #[inline]
    pub fn get_elements(&self) -> (&Elements, &Vec<Vec<Vec<u8>>>) {
        (
            &self.header.elements,
            &self.payload.try_unwrap_scalar_ref().unwrap().data,
        )
    }

    #[inline]
    pub fn get_property(
        &self,
        element_name: &str,
        property_name: &str,
    ) -> Option<(&Property, &Vec<u8>)> {
        let (element, data) = self.get_element(element_name)?;
        let (index, _, property) = element.get_full(property_name)?;
        Some((property, data.get(index)?))
    }

    #[doc(alias = "element")]
    pub fn get_properties(
        &self,
        element_name: &str,
    ) -> Option<(&Properties, &Vec<Vec<u8>>)> {
        let (element, data) = self.get_element(element_name)?;
        Some((&element.properties, data))
    }
}

impl Object {
    #[inline]
    pub fn get_property_as<T: Pod>(
        &self,
        element_name: &str,
        property_name: &str,
    ) -> Option<(&Property, &[T])> {
        let (property, data) = self.get_property(element_name, property_name)?;
        Some((property, try_cast_slice(data).ok()?))
    }
}
