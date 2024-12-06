//! Polygon object access implementation.

pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

/// Element entry.
#[derive(AsRef, Clone, Constructor, Debug, Eq, From, PartialEq)]
pub struct ElementEntry<'e> {
    /// Element metadata.
    pub meta: &'e Element,
    /// Element data.
    pub data: &'e Vec<Vec<u8>>,
}

/// Element entry (mutable).
#[derive(AsRef, Debug, Eq, From, PartialEq)]
pub struct ElementEntryMut<'e> {
    /// Element metadata (mutable).
    pub meta: &'e mut Element,
    /// Element data (mutable).
    pub data: &'e mut Vec<Vec<u8>>,
}

/// Property entry.
#[derive(AsRef, Clone, Constructor, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntry<'p> {
    /// Property metadata.
    pub meta: &'p Property,
    /// Property data.
    pub data: &'p Vec<u8>,
}

/// Property entry (mutable).
#[derive(AsRef, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntryMut<'p> {
    /// Property metadata (mutable).
    pub meta: &'p mut Property,
    /// Property data (mutable).
    pub data: &'p mut Vec<u8>,
}

/// Short named accessors and mutators
impl Object {
    /// Get an element by name.
    #[doc(alias = "get_element")]
    #[inline]
    pub fn elem<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<ElementEntry<'_>> {
        self.get_element(name)
    }

    /// Get a mutable element by name.
    #[doc(alias = "get_mut_element")]
    #[inline]
    pub fn elem_mut<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<ElementEntryMut<'_>> {
        self.get_mut_element(name)
    }

    /// Return an iterator over elements.
    #[doc(alias = "iter_elements")]
    #[inline]
    pub fn elems(&self) -> impl Iterator<Item = ElementEntry<'_>> {
        self.iter_elements()
    }

    /// Return an iterator over mutable elements.
    #[doc(alias = "iter_mut_elements")]
    #[inline]
    pub fn elems_mut(&mut self) -> impl Iterator<Item = ElementEntryMut<'_>> {
        self.iter_mut_elements()
    }

    /// Get a property of an element.
    #[doc(alias = "get_property_of_element")]
    #[inline]
    pub fn elem_prop<'e, 'p: 'e, Q: AsRef<str>>(
        &'p self,
        element_name: Q,
        property_name: Q,
    ) -> Option<PropertyEntry<'p>> {
        self.get_property_of_element(element_name, property_name)
    }

    /// Get a mutable property of an element.
    #[doc(alias = "get_mut_property_of_element")]
    #[inline]
    pub fn elem_prop_mut<'e, 'p: 'e, Q: AsRef<str>>(
        &'p mut self,
        element_name: Q,
        property_name: Q,
    ) -> Option<PropertyEntryMut<'p>> {
        self.get_mut_property_of_element(element_name, property_name)
    }
}

/// Short named accessors
impl<'e, 'p: 'e> ElementEntry<'e> {
    /// Get a property by name.
    #[doc(alias = "get_property")]
    #[inline]
    pub fn prop<Q: AsRef<str>>(
        &'p self,
        name: Q,
    ) -> Option<PropertyEntry<'p>> {
        self.get_property(name)
    }

    /// Return an iterator over properties.
    #[doc(alias = "iter_properties")]
    #[inline]
    pub fn props(&'p self) -> impl Iterator<Item = PropertyEntry<'p>> {
        self.iter_properties()
    }
}

/// Short named mutators
impl<'e, 'p: 'e> ElementEntryMut<'e> {
    /// Get a mutable property by name.
    #[doc(alias = "get_mut_property")]
    #[inline]
    pub fn prop_mut<Q: AsRef<str>>(
        &'p mut self,
        name: Q,
    ) -> Option<PropertyEntryMut<'p>> {
        self.get_mut_property(name)
    }

    /// Return an iterator over mutable properties.
    #[doc(alias = "iter_mut_properties")]
    #[inline]
    pub fn props_mut(&'p mut self) -> impl Iterator<Item = PropertyEntryMut<'p>> {
        self.iter_mut_properties()
    }
}

/// Short named accessors
impl<'p> PropertyEntry<'p> {
    /// Cast the property data to a slice of the kind.
    #[doc(alias = "as_kind")]
    #[inline]
    pub fn cast<T: Pod>(&'p self) -> Result<&'p [T], Error> {
        self.as_kind()
    }
}

/// Short named mutators
impl<'p> PropertyEntryMut<'p> {
    /// Cast the property data to a mutable slice of the kind.
    #[doc(alias = "as_mut_kind")]
    #[inline]
    pub fn cast_mut<T: Pod>(&'p mut self) -> Result<&'p mut [T], Error> {
        self.as_mut_kind()
    }
}

/// Long named accessors and mutators
impl Object {
    /// Get an element by name.
    #[doc(alias = "elem")]
    pub fn get_element<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<ElementEntry<'_>> {
        let (index, _, meta) = self.header.get_full(name.as_ref())?;
        // NOTE: Currently, there is only scalar payload implemented.
        let data = self
            .payload
            .try_unwrap_scalar_ref()
            .unwrap()
            .data
            .get(index)?;
        Some(ElementEntry { meta, data })
    }

    /// Get a mutable element by name.
    #[doc(alias = "elem_mut")]
    pub fn get_mut_element<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<ElementEntryMut<'_>> {
        let (index, _, meta) = self.header.get_full_mut(name.as_ref())?;
        // NOTE: Currently, there is only scalar payload implemented.
        let data = self
            .payload
            .try_unwrap_scalar_mut()
            .unwrap()
            .data
            .get_mut(index)?;
        Some(ElementEntryMut { meta, data })
    }

    /// Get a property of an element.
    #[doc(alias = "elem_prop")]
    pub fn get_property_of_element<'e, 'p: 'e, Q: AsRef<str>>(
        &'p self,
        element_name: Q,
        property_name: Q,
    ) -> Option<PropertyEntry<'p>> {
        let (index, _, meta) = self.header.get_full(element_name.as_ref())?;
        // NOTE: Currently, there is only scalar payload implemented.
        let data = self
            .payload
            .try_unwrap_scalar_ref()
            .unwrap()
            .data
            .get(index)?;
        let (index, _, meta) = meta.get_full(property_name.as_ref())?;
        let data = data.get(index)?;
        Some(PropertyEntry { meta, data })
    }

    /// Get a mutable property of an element.
    #[doc(alias = "elem_prop_mut")]
    pub fn get_mut_property_of_element<'e, 'p: 'e, Q: AsRef<str>>(
        &'p mut self,
        element_name: Q,
        property_name: Q,
    ) -> Option<PropertyEntryMut<'p>> {
        let (index, _, meta) = self.header.get_full_mut(element_name.as_ref())?;
        // NOTE: Currently, there is only scalar payload implemented.
        let data = self
            .payload
            .try_unwrap_scalar_mut()
            .unwrap()
            .data
            .get_mut(index)?;
        let (index, _, meta) = meta.get_full_mut(property_name.as_ref())?;
        let data = data.get_mut(index)?;
        Some(PropertyEntryMut { meta, data })
    }

    /// Return an iterator over elements.
    #[doc(alias = "elems")]
    #[inline]
    pub fn iter_elements(&self) -> impl Iterator<Item = ElementEntry<'_>> {
        // NOTE: Currently, there is only scalar payload implemented.
        self.header
            .elements
            .values()
            .zip(self.payload.try_unwrap_scalar_ref().unwrap().data.iter())
            .map(Into::into)
    }

    /// Return an iterator over mutable elements.
    #[doc(alias = "elems_mut")]
    #[inline]
    pub fn iter_mut_elements(&mut self) -> impl Iterator<Item = ElementEntryMut<'_>> {
        // NOTE: Currently, there is only scalar payload implemented.
        self.header
            .elements
            .values_mut()
            .zip(
                self.payload
                    .try_unwrap_scalar_mut()
                    .unwrap()
                    .data
                    .iter_mut(),
            )
            .map(Into::into)
    }
}

/// Long named accessors
impl<'e, 'p: 'e> ElementEntry<'e> {
    /// Get a property by name.
    #[doc(alias = "prop")]
    #[inline]
    pub fn get_property<Q: AsRef<str>>(
        &'p self,
        name: Q,
    ) -> Option<PropertyEntry<'p>> {
        let (index, _, meta) = self.meta.get_full(name.as_ref())?;
        let data = self.data.get(index)?;
        Some(PropertyEntry { meta, data })
    }

    /// Return an iterator over properties.
    #[doc(alias = "props")]
    #[inline]
    pub fn iter_properties(&'p self) -> impl Iterator<Item = PropertyEntry<'p>> {
        self.meta
            .properties
            .values()
            .zip(self.data.iter())
            .map(Into::into)
    }
}

/// Long named mutators
impl<'e, 'p: 'e> ElementEntryMut<'e> {
    /// Get a mutable property by name.
    #[doc(alias = "prop_mut")]
    #[inline]
    pub fn get_mut_property<Q: AsRef<str>>(
        &'p mut self,
        name: Q,
    ) -> Option<PropertyEntryMut<'p>> {
        let (index, _, meta) = self.meta.get_full_mut(name.as_ref())?;
        let data = self.data.get_mut(index)?;
        Some(PropertyEntryMut { meta, data })
    }

    /// Return an iterator over mutable properties.
    #[doc(alias = "props_mut")]
    #[inline]
    pub fn iter_mut_properties(
        &'p mut self
    ) -> impl Iterator<Item = PropertyEntryMut<'p>> {
        self.meta
            .properties
            .values_mut()
            .zip(self.data.iter_mut())
            .map(Into::into)
    }
}

/// Long named accessors
impl<'p> PropertyEntry<'p> {
    /// Cast the property data to a slice of the kind.
    #[doc(alias = "cast")]
    #[inline]
    pub fn as_kind<T: Pod>(&'p self) -> Result<&'p [T], Error> {
        Ok(try_cast_slice(self.data)?)
    }
}

/// Long named mutators
impl<'p> PropertyEntryMut<'p> {
    /// Cast the property data to a mutable slice of the kind.
    #[doc(alias = "cast_mut")]
    #[inline]
    pub fn as_mut_kind<T: Pod>(&'p mut self) -> Result<&'p mut [T], Error> {
        Ok(try_cast_slice_mut(self.data)?)
    }
}
