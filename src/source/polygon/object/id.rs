use std::{fmt, ops::Deref};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static ID: AtomicU64 = AtomicU64::new(0);
        Self(ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Deref for Id {
    type Target = u64;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Id {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Id {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug() {
        use super::*;

        let target = true;
        let output = format!("{:#?}", Id::new()).parse::<u64>().is_ok();
        assert_eq!(output, target);
    }

    #[test]
    fn default_and_deref() {
        use super::*;

        let target: u64 = *Id::new() + 1;
        let output: u64 = *Id::default();
        assert_eq!(output, target);
    }
}
