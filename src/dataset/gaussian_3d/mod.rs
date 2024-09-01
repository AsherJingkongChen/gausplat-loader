pub mod camera;
pub mod image;
pub mod point;
pub mod view;

pub use camera::*;
pub use image::*;
pub use point::*;
pub use view::*;

use std::fmt;

#[derive(Clone, Default, PartialEq)]
pub struct Gaussian3dDataset {
    pub cameras: Cameras,
    pub points: Points,
}

impl fmt::Debug for Gaussian3dDataset {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Gaussian3dDataset")
            .field("cameras.len()", &self.cameras.len())
            .field("points.len()", &self.points.len())
            .finish()
    }
}
