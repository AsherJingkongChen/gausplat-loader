pub mod views;

use std::fmt;
pub use views::*;

#[derive(Clone, PartialEq)]
pub struct View {
    pub focal_length_x: f64,
    pub focal_length_y: f64,
    pub image: image::RgbImage,
    pub(crate) image_file_name: String,
    pub position: [f64; 3],
    pub projection_transform: [[f64; 4]; 4],
    pub(crate) view_id: u32,
    pub view_transform: [[f64; 4]; 4],
}

impl View {
    pub fn image_file_name(&self) -> &str {
        &self.image_file_name
    }

    pub fn view_id(&self) -> &u32 {
        &self.view_id
    }
}

impl fmt::Debug for View {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("View")
            .field("image_file_name", &self.image_file_name)
            .field("image_height", &self.image.height())
            .field("image_width", &self.image.width())
            .field("projection_transform", &self.projection_transform)
            .field("view_id", &self.view_id)
            .field("view_transform", &self.view_transform)
            .finish()
    }
}
