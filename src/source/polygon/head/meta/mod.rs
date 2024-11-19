pub mod comment;
pub mod element;
pub mod format;
pub mod property;

pub use super::*;
pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};
pub use comment::*;
pub use element::*;
pub use format::*;
pub use property::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Meta {
    pub id: Id,
    pub variant: MetaVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MetaVariant {
    Comment(CommentMeta),
    Element(ElementMeta),
    ObjInfo(ObjInfoMeta),
    Property(PropertyMeta),
}

impl_variant_matchers!(Meta, Comment, Element, ObjInfo, Property);

// TODO: Reduce tests
#[cfg(test)]
mod tests {
    #[test]
    fn matcher_as() {
        use super::*;

        let target = true;
        let output = MetaVariant::Comment(Default::default())
            .as_comment()
            .is_some();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::Element(Default::default())
            .as_element()
            .is_some();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::ObjInfo(Default::default())
            .as_obj_info()
            .is_some();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::Property(Default::default())
            .as_property()
            .is_some();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Comment(Default::default())
            .as_element()
            .is_some();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Element(Default::default())
            .as_property()
            .is_some();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Property(Default::default())
            .as_comment()
            .is_some();
        assert_eq!(output, target);
    }

    #[test]
    fn matcher_is() {
        use super::*;

        let target = true;
        let output = MetaVariant::Comment(Default::default()).is_comment();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::Element(Default::default()).is_element();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::ObjInfo(Default::default()).is_obj_info();
        assert_eq!(output, target);

        let target = true;
        let output = MetaVariant::Property(Default::default()).is_property();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Comment(Default::default()).is_element();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Element(Default::default()).is_property();
        assert_eq!(output, target);

        let target = false;
        let output = MetaVariant::Property(Default::default()).is_comment();
        assert_eq!(output, target);
    }
}
