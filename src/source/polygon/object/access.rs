pub use super::*;
pub use bytemuck::Pod;

use bytemuck::{try_cast_slice, try_cast_slice_mut};

#[derive(AsRef, Clone, Constructor, Debug, Eq, From, PartialEq)]
pub struct ElementEntry<'e> {
    pub element: &'e Element,
    pub data: &'e Vec<Vec<u8>>,
}

#[derive(AsRef, Debug, Eq, From, PartialEq)]
pub struct ElementEntryMut<'e> {
    pub element: &'e mut Element,
    pub data: &'e mut Vec<Vec<u8>>,
}

#[derive(AsRef, Clone, Constructor, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntry<'p> {
    pub property: &'p Property,
    pub data: &'p Vec<u8>,
}

#[derive(AsRef, Debug, Eq, From, Hash, PartialEq)]
pub struct PropertyEntryMut<'p> {
    pub property: &'p mut Property,
    pub data: &'p mut Vec<u8>,
}

// Short name accessors
impl Object {
    #[doc(alias = "get_element")]
    #[inline]
    pub fn elem<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<ElementEntry> {
        self.get_element(name)
    }

    #[doc(alias = "get_mut_element")]
    #[inline]
    pub fn elem_mut<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<ElementEntryMut> {
        self.get_mut_element(name)
    }

    #[doc(alias = "iter_elements")]
    #[inline]
    pub fn elems(&self) -> impl Iterator<Item = ElementEntry> {
        self.iter_elements()
    }

    #[doc(alias = "iter_mut_elements")]
    #[inline]
    pub fn elems_mut(&mut self) -> impl Iterator<Item = ElementEntryMut> {
        self.iter_mut_elements()
    }
}

impl ElementEntry<'_> {
    #[doc(alias = "get_property")]
    #[inline]
    pub fn prop<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<PropertyEntry> {
        self.get_property(name)
    }

    #[doc(alias = "iter_properties")]
    #[inline]
    pub fn props(&self) -> impl Iterator<Item = PropertyEntry> {
        self.iter_properties()
    }
}

impl ElementEntryMut<'_> {
    #[doc(alias = "get_mut_property")]
    #[inline]
    pub fn prop_mut<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<PropertyEntryMut> {
        self.get_mut_property(name)
    }

    #[doc(alias = "iter_mut_properties")]
    #[inline]
    pub fn props_mut(&mut self) -> impl Iterator<Item = PropertyEntryMut> {
        self.iter_mut_properties()
    }
}

impl PropertyEntry<'_> {
    #[doc(alias = "as_kind")]
    #[inline]
    pub fn cast<T: Pod>(&self) -> Option<&[T]> {
        self.as_kind()
    }
}

impl PropertyEntryMut<'_> {
    #[doc(alias = "as_mut_kind")]
    #[inline]
    pub fn cast_mut<T: Pod>(&mut self) -> Option<&mut [T]> {
        self.as_mut_kind()
    }
}

// Full name accessors

impl Object {
    #[doc(alias = "elem")]
    #[inline]
    pub fn get_element<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<ElementEntry> {
        let (index, _, element) = self.header.get_full(name.as_ref())?;
        let data = self
            .payload
            .try_unwrap_scalar_ref()
            .unwrap()
            .data
            .get(index)?;
        Some(ElementEntry { element, data })
    }

    #[doc(alias = "elem_mut")]
    #[inline]
    pub fn get_mut_element<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<ElementEntryMut> {
        let (index, _, element) = self.header.get_full_mut(name.as_ref())?;
        let data = self
            .payload
            .try_unwrap_scalar_mut()
            .unwrap()
            .data
            .get_mut(index)?;
        Some(ElementEntryMut { element, data })
    }

    #[doc(alias = "elems")]
    #[inline]
    pub fn iter_elements(&self) -> impl Iterator<Item = ElementEntry> {
        self.header
            .elements
            .values()
            .zip(self.payload.try_unwrap_scalar_ref().unwrap().data.iter())
            .map(Into::into)
    }

    #[doc(alias = "elems_mut")]
    #[inline]
    pub fn iter_mut_elements(&mut self) -> impl Iterator<Item = ElementEntryMut> {
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

impl ElementEntry<'_> {
    #[doc(alias = "prop")]
    #[inline]
    pub fn get_property<Q: AsRef<str>>(
        &self,
        name: Q,
    ) -> Option<PropertyEntry> {
        let (index, _, property) = self.element.get_full(name.as_ref())?;
        let data = self.data.get(index)?;
        Some(PropertyEntry { property, data })
    }

    #[doc(alias = "props")]
    #[inline]
    pub fn iter_properties(&self) -> impl Iterator<Item = PropertyEntry> {
        self.element
            .properties
            .values()
            .zip(self.data.iter())
            .map(Into::into)
    }
}

impl ElementEntryMut<'_> {
    #[doc(alias = "prop_mut")]
    #[inline]
    pub fn get_mut_property<Q: AsRef<str>>(
        &mut self,
        name: Q,
    ) -> Option<PropertyEntryMut> {
        let (index, _, property) = self.element.get_full_mut(name.as_ref())?;
        let data = self.data.get_mut(index)?;
        Some(PropertyEntryMut { property, data })
    }

    #[doc(alias = "props_mut")]
    #[inline]
    pub fn iter_mut_properties(&mut self) -> impl Iterator<Item = PropertyEntryMut> {
        self.element
            .properties
            .values_mut()
            .zip(self.data.iter_mut())
            .map(Into::into)
    }
}

impl PropertyEntry<'_> {
    #[inline]
    pub fn as_kind<T: Pod>(&self) -> Option<&[T]> {
        try_cast_slice(self.data).ok()
    }
}

impl PropertyEntryMut<'_> {
    #[inline]
    pub fn as_mut_kind<T: Pod>(&mut self) -> Option<&mut [T]> {
        try_cast_slice_mut(self.data).ok()
    }
}
