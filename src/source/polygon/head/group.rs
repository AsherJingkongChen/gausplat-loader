pub use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Group {
    element_id_to_property_ids: IndexMap<Id, Vec<Id>>,
    property_id_to_element_id: IndexMap<Id, Id>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GroupBuilder {
    pub element: Option<ElementMeta>,
    pub element_id: Option<Id>,
    pub element_id_and_property_id: Vec<(Id, Id)>,
}

impl Group {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            element_id_to_property_ids: IndexMap::with_capacity(2),
            property_id_to_element_id: IndexMap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn get_element_id(
        &self,
        property_id: Id,
    ) -> Option<Id> {
        self.property_id_to_element_id.get(&property_id).copied()
    }

    #[inline]
    pub fn get_property_ids(
        &self,
        element_id: Id,
    ) -> Option<&[Id]> {
        self.element_id_to_property_ids
            .get(&element_id)
            .map(AsRef::as_ref)
    }

    #[inline]
    pub fn set_property_id(
        &mut self,
        property_id: Id,
        element_id: Id,
    ) -> &mut Self {
        self.element_id_to_property_ids
            .entry(element_id)
            .or_insert_with(|| Vec::with_capacity(8))
            .push(property_id);
        self.property_id_to_element_id
            .insert(property_id, element_id);
        self
    }

    #[inline]
    pub fn iter_element_id_and_property_ids(
        &self
    ) -> impl Iterator<Item = (&Id, &[Id])> {
        self.element_id_to_property_ids.iter().map(
            |(element_id, property_ids)| (element_id, property_ids.as_ref()),
        )
    }
}

impl GroupBuilder {
    #[inline]
    pub fn add_property_id(
        &mut self,
        property_id: Id,
    ) -> Option<()> {
        self.element_id_and_property_id
            .push((self.element_id?, property_id));
        Some(())
    }

    #[inline]
    pub fn take_element(&mut self) -> Option<ElementMeta> {
        self.element.take()
    }

    #[inline]
    pub fn set_element(
        &mut self,
        element: ElementMeta,
    ) {
        self.element = Some(element);
    }

    #[inline]
    pub fn set_element_id(
        &mut self,
        element_id: Id,
    ) {
        self.element_id = Some(element_id);
    }

    #[inline]
    pub fn build(self) -> Group {
        let property_count = self.element_id_and_property_id.len();

        self.element_id_and_property_id.into_iter().fold(
            Group::with_capacity(property_count),
            |mut group, (element_id, property_id)| {
                group.set_property_id(property_id, element_id);
                group
            },
        )
    }
}

impl Default for GroupBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            element: None,
            element_id: None,
            element_id_and_property_id: Vec::with_capacity(8),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn build_on_simple_tree() {
        use super::*;

        let source_element_id = Id::new();
        let source_property_ids = [Id::new(), Id::new()];
        let mut group = GroupBuilder::default();
        group.set_element_id(source_element_id);
        group.add_property_id(source_property_ids[0]).unwrap();
        group.add_property_id(source_property_ids[1]).unwrap();
        let group = group.build();

        let target = source_property_ids[1];
        let output = group.get_property_ids(source_element_id).unwrap()[1];
        assert_eq!(output, target);

        let target = source_element_id;
        let output = group.get_element_id(source_property_ids[0]).unwrap();
        assert_eq!(output, target);
        let output = group.get_element_id(source_property_ids[1]).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn build_on_misplaced_property() {
        use super::*;

        let target = None;
        let output = GroupBuilder::default().add_property_id(Id::new());
        assert_eq!(output, target);
    }

    #[test]
    fn get_and_set_property_id() {
        use super::*;

        let mut group = Group::default();
        let element_id = Id::new();
        let property_id = Id::new();
        group.set_property_id(property_id, element_id);

        let target = element_id;
        let output = group.get_element_id(property_id).unwrap();
        assert_eq!(output, target);

        let target = &[property_id];
        let output = group.get_property_ids(element_id).unwrap();
        assert_eq!(output, target);
    }
}
