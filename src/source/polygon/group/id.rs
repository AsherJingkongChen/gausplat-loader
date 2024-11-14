use std::{fmt, ops::Deref};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

impl fmt::Display for Id {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<u64> for Id {
    #[inline]
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for Id {
    #[inline]
    fn into(self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_and_deref_and_into() {
        use super::*;

        let _: u64 = Id::default().into();

        let target: u64 = (*Id::new() + 1).into();
        let output: usize = Id::default().into();
        assert_eq!(output as u64, target);
    }

    #[test]
    fn display() {
        use super::*;

        let target = true;
        let output =
            u64::from_str_radix(&format!("{}", Id::default()), 10).is_ok();
        assert_eq!(output, target);
    }
}
