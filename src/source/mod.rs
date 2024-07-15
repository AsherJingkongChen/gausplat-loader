pub mod colmap;
pub mod point;
pub mod view;

use crate::error::*;
pub use colmap::ColmapSource;
pub use point::*;
use std::collections::HashMap;
pub use view::*;

pub trait Source {
    fn read_points(&mut self) -> Result<Vec<Point>, Error>;
    fn read_views(&mut self) -> Result<HashMap<u32, View>, Error>;
}
