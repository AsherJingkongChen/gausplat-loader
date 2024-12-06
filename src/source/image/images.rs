//! A collection of images.

pub use super::Image;

/// A map of [`Image::image_id`] to [`Image`].
pub type Images = crate::collection::IndexMap<u32, Image>;
