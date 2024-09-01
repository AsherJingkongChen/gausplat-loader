pub mod image;
pub mod point;
pub mod view;

pub use image::*;
pub use point::*;
pub use view::*;

use std::fmt;

#[derive(Clone, Default, PartialEq)]
pub struct Gaussian3dSceneData {
    /// The image id is also the view id
    pub images: Images,
    pub points: Points,
    /// The view id is also the image id
    pub views: Views,
}

impl fmt::Debug for Gaussian3dSceneData {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Gaussian3dSceneData")
            .field("images.len()", &self.images.len())
            .field("points.len()", &self.points.len())
            .field("views.len()", &self.views.len())
            .finish()
    }
}
