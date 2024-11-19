pub mod list;
pub mod scalar;

pub use super::*;
pub use list::*;
pub use scalar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub id: Id,
    pub variant: DataVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataVariant {
    List(ListData),
    Scalar(ScalarData),
}

impl_variant_matchers!(Data, List, Scalar);

// TODO: Reduce tests
#[cfg(test)]
mod tests {
    #[test]
    fn matcher_as() {
        use super::*;

        let target = true;
        let output = DataVariant::List(Default::default()).as_list().is_some();
        assert_eq!(output, target);

        let target = true;
        let output = DataVariant::Scalar(Default::default())
            .as_scalar()
            .is_some();
        assert_eq!(output, target);

        let target = false;
        let output =
            DataVariant::List(Default::default()).as_scalar().is_some();
        assert_eq!(output, target);
    }

    #[test]
    fn matcher_is() {
        use super::*;

        let target = true;
        let output = DataVariant::List(Default::default()).is_list();
        assert_eq!(output, target);

        let target = true;
        let output = DataVariant::Scalar(Default::default()).is_scalar();
        assert_eq!(output, target);

        let target = false;
        let output = DataVariant::List(Default::default()).is_scalar();
        assert_eq!(output, target);
    }
}
