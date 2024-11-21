pub mod data;
pub mod info;

pub use super::*;
pub use data::*;
pub use info::*;

use std::fmt;

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListPropertyBlock {
    pub data: ListPropertyBlockData,
    pub info: ListPropertyBlockInfo,
}

impl fmt::Debug for ListPropertyBlock {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{{ {:?} [{:?}] }}", self.info, self.data.len())
    }
}

impl ops::Deref for ListPropertyBlock {
    type Target = ListPropertyBlockInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl ops::DerefMut for ListPropertyBlock {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug() {
        use super::*;

        let block = ListPropertyBlock {
            data: ListPropertyBlockData::from([[].into(), [].into(), [].into()]),
            info: ListPropertyBlockInfo {
                count: ScalarPropertyBlockInfo::new("uint", 4).unwrap(),
                value: INT32.to_owned(),
            },
        };
        assert_eq!(format!("{:?}", block), r#"{ uint <> int32 [3] }"#);
    }
}
