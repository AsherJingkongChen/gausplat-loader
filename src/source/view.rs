use std::collections::HashMap;
use std::fmt;

pub type Views = HashMap<u32, View>;

#[derive(Clone, PartialEq)]
pub struct View {
    pub view_transform: [[f64; 4]; 4],
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image: image::RgbImage,
    pub(super) image_file_name: String,
    pub(super) view_id: u32,
    pub view_height: u64,
    pub view_width: u64,
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
            .field("field_of_view_x", &self.field_of_view_x)
            .field("field_of_view_y", &self.field_of_view_y)
            .field("image_file_name", &self.image_file_name)
            .field("image_height", &self.image.height())
            .field("image_width", &self.image.width())
            .field("view_id", &self.view_id)
            .field("view_height", &self.view_height)
            .field("view_width", &self.view_width)
            .field("view_transform", &self.view_transform)
            .finish()
    }
}
