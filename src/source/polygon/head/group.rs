pub use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HeadGroup {
    pub element_to_property_id: IndexMap<Id, Vec<Id>>,
    pub property_to_element_id: IndexMap<Id, Id>,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HeadGroupBuilder {
    pub element_id: Option<Id>,
    pub element_id_and_property_id: Vec<(Id, Id)>,
}

impl HeadGroup {
    #[inline]
    pub fn get_element_id(
        &self,
        property_id: Id,
    ) -> Option<Id> {
        self.property_to_element_id.get(&property_id).copied()
    }

    #[inline]
    pub fn get_property_ids(
        &self,
        element_id: Id,
    ) -> Option<&[Id]> {
        self.element_to_property_id
            .get(&element_id)
            .map(AsRef::as_ref)
    }

    #[inline]
    pub fn set_property_id(
        &mut self,
        property_id: Id,
        element_id: Id,
    ) -> &mut Self {
        self.element_to_property_id
            .entry(element_id)
            .or_insert_with(vec_default_small)
            .push(property_id);
        self.property_to_element_id.insert(property_id, element_id);
        self
    }
}

impl HeadGroupBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_property_id(
        mut self,
        property_id: Id,
    ) -> Option<Self> {
        self.element_id_and_property_id
            .push((self.element_id?, property_id));
        Some(self)
    }

    #[inline]
    pub fn set_element_id(
        mut self,
        element_id: Id,
    ) -> Self {
        self.element_id = Some(element_id);
        self
    }

    #[inline]
    pub fn build(self) -> HeadGroup {
        self.element_id_and_property_id.into_iter().fold(
            HeadGroup::default(),
            |mut group, (element_id, property_id)| {
                group.set_property_id(property_id, element_id);
                group
            },
        )
    }
}

#[inline]
fn vec_default_small<T>() -> Vec<T> {
    Vec::with_capacity(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_new() {
        let target = HeadGroup::default();
        let output = HeadGroupBuilder::new().build();
        assert_eq!(output, target);
    }

    #[test]
    fn build_on_simple_tree() {
        let source_element_id = Id::new();
        let source_property_ids = [Id::new(), Id::new()];
        let group = HeadGroupBuilder::default()
            .set_element_id(source_element_id)
            .add_property_id(source_property_ids[0])
            .unwrap()
            .add_property_id(source_property_ids[1])
            .unwrap()
            .build();

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
    fn build_on_orphan_property() {
        let target = None;
        let output = HeadGroupBuilder::default().add_property_id(Id::new());
        assert_eq!(output, target);
    }

    #[test]
    fn get_and_set_property_id() {
        let mut group = HeadGroup::default();
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
