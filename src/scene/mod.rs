pub mod point;
pub mod view;

use std::fmt;

pub use point::*;
pub use view::*;

#[derive(Clone, PartialEq)]
pub struct Scene {
    pub points: Points,
    pub views: Views,
}

impl fmt::Debug for Scene {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Scene")
            .field("points.len()", &self.points.len())
            .field("views.len()", &self.views.len())
            .finish()
    }
}