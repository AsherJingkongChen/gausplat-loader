pub use crate::error::Error;

use std::path;

pub trait Opener
where
    Self: Sized,
{
    fn open<P: AsRef<path::Path>>(path: P) -> Result<Self, Error>;
}
