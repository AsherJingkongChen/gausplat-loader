pub use crate::error::Error;

use std::path::Path;

pub trait Opener
where
    Self: Sized,
{
    fn open(path: impl AsRef<Path>) -> Result<Self, Error>;
}
