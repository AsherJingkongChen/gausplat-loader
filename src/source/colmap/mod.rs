pub mod camera;
pub mod image;
pub mod point;

pub use super::file::*;
pub use camera::*;
pub use image::*;
pub use point::*;

use std::fmt;

#[derive(Clone, PartialEq)]
pub struct ColmapSource<S> {
    pub cameras: Cameras,
    pub images: Images,
    pub images_file: Files<S>,
    pub points: Points,
}

impl<S> fmt::Debug for ColmapSource<S> {
    #[inline]
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
    #[inline]
    fn default() -> Self {
        Self {
            cameras: Default::default(),
            images_file: Default::default(),
            images: Default::default(),
            points: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_and_default() {
        use super::*;

        let target = ColmapSource::<&[u8]> {
            cameras: Default::default(),
            images: Default::default(),
            images_file: Default::default(),
            points: Default::default(),
        };
        let output = ColmapSource::<&[u8]>::default();
        assert_eq!(output, target);

        let target = true;
        let output = format!("{:?}", output).starts_with("ColmapSource");
        assert_eq!(output, target);
    }
}
