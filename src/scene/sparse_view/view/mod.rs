pub mod views;

pub use views::*;

use std::fmt;

#[derive(Clone, Default, PartialEq)]
pub struct View {
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image_file_name: String,
    pub image_height: u32,
    pub image_width: u32,
    pub view_id: u32,

    /// The position of the view in world space
    pub view_position: [f64; 3],

    /// The transformation matrix from world space to view space in **column-major** order
    pub view_transform: [[f64; 4]; 4],
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
            .field("image_height", &self.image_height)
            .field("image_width", &self.image_width)
            .field("view_id", &self.view_id)
            .field("view_position", &self.view_position)
            .field("view_transform", &self.view_transform)
            .finish()
    }
}
