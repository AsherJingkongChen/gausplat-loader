pub mod image;
pub mod point;
pub mod view;

pub use image::*;
pub use point::*;
pub use view::*;

use std::fmt;

#[derive(Clone, PartialEq)]
pub struct SparseViewScene {
    pub images: Images,
    pub points: Points,
    pub views: Views,
}

impl fmt::Debug for SparseViewScene {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Scene")
            .field("images.len()", &self.images.len())
            .field("points.len()", &self.points.len())
            .field("views.len()", &self.views.len())
            .finish()
    }
}
