pub mod views;

pub use views::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct View {
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image_height: u32,
    pub image_width: u32,
    pub view_id: u32,

    /// The view position in world space.
    pub view_position: [f64; 3],

    /// The affine transformation matrix mapping
    /// from world space to view space **(in column-major order)**.
    pub view_transform: [[f64; 4]; 4],
}
