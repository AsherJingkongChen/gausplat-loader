pub mod colmap;
pub mod point;
pub mod view;

use crate::error::*;
pub use colmap::ColmapSource;
pub use point::*;
pub use view::*;

pub trait Source {
    fn read_points(&mut self) -> Result<Points, Error>;
    fn read_views(&mut self) -> Result<Views, Error>;
}
