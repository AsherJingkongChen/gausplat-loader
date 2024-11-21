pub mod data;
pub mod info;

pub use super::*;
pub use data::*;
pub use info::*;

use std::fmt;

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarPropertyBlock {
    pub data: ScalarPropertyBlockData,
    pub info: ScalarPropertyBlockInfo,
}

impl fmt::Debug for ScalarPropertyBlock {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{{ {:?} [{:?}] }}",
            self.info,
            self.data.len() / self.info.step
        )
    }
}

impl ops::Deref for ScalarPropertyBlock {
    type Target = ScalarPropertyBlockInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn debug() {
        use super::*;

        let block = ScalarPropertyBlock {
            data: ScalarPropertyBlockData::from([0, 0, 0, 0, 0, 0, 0, 0]),
            info: UINT32.to_owned(),
        };
        assert_eq!(format!("{:?}", block), r#"{ uint32 [2] }"#);
    }
}
