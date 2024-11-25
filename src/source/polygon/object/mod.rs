pub mod decode;
pub mod encode;

pub use super::*;
pub use bytemuck::Pod;
pub use bytemuck::{try_cast_slice, try_cast_slice_mut};
pub use header::*;

use derive_more::derive::{AsRef, Constructor, Display, From};
use Error::*;

#[derive(AsRef, Clone, Constructor, Debug, Default, Display, Eq, From, PartialEq)]
#[display("{header}----------\n{payload}")]
pub struct Object {
    pub header: Header,
    pub payload: Payload,
}

impl Object {
    #[doc(alias = "get_properties")]
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

    #[doc(alias = "get_element")]
    pub fn get_properties(
        &self,
        element_name: &str,
    ) -> Option<(&Properties, &Vec<Vec<u8>>)> {
        let (element, data) = self.get_element(element_name)?;
        Some((&element.properties, data))
    }

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

impl Object {
    #[doc(alias = "get_mut_properties")]
    #[inline]
    pub fn get_mut_element(
        &mut self,
        element_name: &str,
    ) -> Option<(&mut Element, &mut Vec<Vec<u8>>)> {
        let (index, _, element) = self.header.get_full_mut(element_name)?;
        Some((
            element,
            self.payload
                .try_unwrap_scalar_mut()
                .unwrap()
                .data
                .get_mut(index)?,
        ))
    }

    #[inline]
    pub fn get_mut_elements(&mut self) -> (&mut Elements, &mut Vec<Vec<Vec<u8>>>) {
        (
            &mut self.header.elements,
            &mut self.payload.try_unwrap_scalar_mut().unwrap().data,
        )
    }

    #[inline]
    pub fn get_mut_property(
        &mut self,
        element_name: &str,
        property_name: &str,
    ) -> Option<(&mut Property, &mut Vec<u8>)> {
        let (element, data) = self.get_mut_element(element_name)?;
        let (index, _, property) = element.get_full_mut(property_name)?;
        Some((property, data.get_mut(index)?))
    }

    #[doc(alias = "get_mut_element")]
    pub fn get_mut_properties(
        &mut self,
        element_name: &str,
    ) -> Option<(&mut Properties, &mut Vec<Vec<u8>>)> {
        let (element, data) = self.get_mut_element(element_name)?;
        Some((&mut element.properties, data))
    }

    #[inline]
    pub fn get_mut_property_as<T: Pod>(
        &mut self,
        element_name: &str,
        property_name: &str,
    ) -> Option<(&mut Property, &mut [T])> {
        let (property, data) = self.get_mut_property(element_name, property_name)?;
        Some((property, try_cast_slice_mut(data).ok()?))
    }
}
