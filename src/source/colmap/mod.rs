pub mod camera;
pub mod image;
pub mod point;

pub use super::file::*;
pub use camera::*;
pub use image::*;
pub use point::*;

use std::fmt;

pub struct ColmapSource<S> {
    pub cameras: Cameras,
    pub images: Images,
    pub images_file: Files<S>,
    pub points: Points,
}

impl<S> fmt::Debug for ColmapSource<S> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ColmapSource")
            .field("cameras.len()", &self.cameras.len())
            .field("images.len()", &self.images.len())
            .field("images_file.len()", &self.images_file.len())
            .field("points.len()", &self.points.len())
            .finish()
    }
}

impl<S: Default> Default for ColmapSource<S> {
    fn default() -> Self {
        Self {
            cameras: Default::default(),
            images_file: Default::default(),
            images: Default::default(),
            points: Default::default(),
        }
    }
}
