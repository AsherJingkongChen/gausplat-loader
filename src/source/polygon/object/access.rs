pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

#[derive(AsRef, Clone, Constructor, Debug, Eq, From, PartialEq)]
pub struct ElementEntry<'e> {
    pub meta: &'e Element,
    pub data: &'e Vec<Vec<u8>>,
}

#[derive(AsRef, Debug, Eq, From, PartialEq)]
pub struct ElementEntryMut<'e> {
    pub meta: &'e mut Element,
    pub data: &'e mut Vec<Vec<u8>>,
}

#[derive(AsRef, Clone, Constructor, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntry<'p> {
    pub meta: &'p Property,
    pub data: &'p Vec<u8>,
}

#[derive(AsRef, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntryMut<'p> {
    pub meta: &'p mut Property,
    pub data: &'p mut Vec<u8>,
}

// Short name accessors
impl Object {
    #[doc(alias = "get_element")]
    #[inline]
    pub fn elem<'e, Q: AsRef<str>>(
        &'e self,
        name: Q,
    ) -> Option<ElementEntry<'e>> {
        self.get_element(name)
    }

    #[doc(alias = "get_mut_element")]
    #[inline]
    pub fn elem_mut<'e, Q: AsRef<str>>(
        &'e mut self,
        name: Q,
    ) -> Option<ElementEntryMut<'e>> {
        self.get_mut_element(name)
    }

    #[doc(alias = "iter_elements")]
    #[inline]
    pub fn elems<'e>(&'e self) -> impl Iterator<Item = ElementEntry<'e>> {
        self.iter_elements()
    }

    #[doc(alias = "iter_mut_elements")]
    #[inline]
    pub fn elems_mut<'e>(&'e mut self) -> impl Iterator<Item = ElementEntryMut<'e>> {
        self.iter_mut_elements()
    }
}

impl<'e, 'p: 'e> ElementEntry<'e> {
    #[doc(alias = "get_property")]
    #[inline]
    pub fn prop<Q: AsRef<str>>(
        &'p self,
        name: Q,
    ) -> Option<PropertyEntry<'p>> {
        self.get_property(name)
    }

    #[doc(alias = "iter_properties")]
    #[inline]
    pub fn props(&'p self) -> impl Iterator<Item = PropertyEntry<'p>> {
        self.iter_properties()
    }
}

impl<'e, 'p: 'e> ElementEntryMut<'e> {
    #[doc(alias = "get_mut_property")]
    #[inline]
    pub fn prop_mut<Q: AsRef<str>>(
        &'p mut self,
        name: Q,
    ) -> Option<PropertyEntryMut<'p>> {
        self.get_mut_property(name)
    }

    #[doc(alias = "iter_mut_properties")]
    #[inline]
    pub fn props_mut(&'p mut self) -> impl Iterator<Item = PropertyEntryMut<'p>> {
        self.iter_mut_properties()
    }
}

impl<'p> PropertyEntry<'p> {
    #[doc(alias = "as_kind")]
    #[inline]
    pub fn cast<T: Pod>(&'p self) -> Result<&'p [T], Error> {
        self.as_kind()
    }
}

impl<'p> PropertyEntryMut<'p> {
    #[doc(alias = "as_mut_kind")]
    #[inline]
    pub fn cast_mut<T: Pod>(&'p mut self) -> Result<&'p mut [T], Error> {
        self.as_mut_kind()
    }
}

// Full name accessors

impl Object {
    #[doc(alias = "elem")]
    #[inline]
    pub fn get_element<'e, Q: AsRef<str>>(
        &'e self,
        name: Q,
    ) -> Option<ElementEntry<'e>> {
        let (index, _, meta) = self.header.get_full(name.as_ref())?;
        let data = self
            .payload
            .try_unwrap_scalar_ref()
            .unwrap()
            .data
            .get(index)?;
        Some(ElementEntry { meta, data })
    }

    #[doc(alias = "elem_mut")]
    #[inline]
    pub fn get_mut_element<'e, Q: AsRef<str>>(
        &'e mut self,
        name: Q,
    ) -> Option<ElementEntryMut<'e>> {
        let (index, _, meta) = self.header.get_full_mut(name.as_ref())?;
        let data = self
            .payload
            .try_unwrap_scalar_mut()
            .unwrap()
            .data
            .get_mut(index)?;
        Some(ElementEntryMut { meta, data })
    }

    #[doc(alias = "elems")]
    #[inline]
    pub fn iter_elements<'e>(&'e self) -> impl Iterator<Item = ElementEntry<'e>> {
        self.header
            .elements
            .values()
            .zip(self.payload.try_unwrap_scalar_ref().unwrap().data.iter())
            .map(Into::into)
    }

    #[doc(alias = "elems_mut")]
    #[inline]
    pub fn iter_mut_elements<'e>(
        &'e mut self
    ) -> impl Iterator<Item = ElementEntryMut<'e>> {
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

impl<'e, 'p: 'e> ElementEntry<'e> {
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

impl<'e, 'p: 'e> ElementEntryMut<'e> {
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

impl<'p> PropertyEntry<'p> {
    #[inline]
    pub fn as_kind<T: Pod>(&'p self) -> Result<&'p [T], Error> {
        Ok(try_cast_slice(self.data)?)
    }
}

impl<'p> PropertyEntryMut<'p> {
    #[inline]
    pub fn as_mut_kind<T: Pod>(&'p mut self) -> Result<&'p mut [T], Error> {
        Ok(try_cast_slice_mut(self.data)?)
    }
}
