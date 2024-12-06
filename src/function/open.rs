//! Functions for opening process.

pub use crate::error::Error;

use std::path::Path;

/// Opening function.
pub trait Opener
where
    Self: Sized,
{
    /// Open the stream from the path.
    fn open(path: impl AsRef<Path>) -> Result<Self, Error>;
}
