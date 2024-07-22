pub mod images;

pub use images::*;

use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Image {
    pub image: image::RgbImage,
    pub(crate) view_id: u32,
}

impl Image {
    pub fn view_id(&self) -> &u32 {
        &self.view_id
    }
}

impl fmt::Debug for Image {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Image")
            .field("height", &self.image.height())
            .field("width", &self.image.width())
            .field("view_id", &self.view_id)
            .finish()
    }
}