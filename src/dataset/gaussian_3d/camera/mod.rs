pub mod cameras;

pub use super::{Image, View};
pub use cameras::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Camera {
    /// `camera_id == self.image.image_id && camera_id == self.view.view_id`
    pub camera_id: u32,
    pub image: Image,
    pub view: View,
}
