pub use crate::error::Error;

use std::path;

pub trait Opener
where
    Self: Sized,
{
    fn open(path: impl AsRef<path::Path>) -> Result<Self, Error>;
}
