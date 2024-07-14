pub mod camera;
pub mod image;
pub mod image_file;
pub mod point;

use std::io;

pub use camera::*;
pub use image::*;
pub use image_file::*;
pub use point::*;

pub struct ColmapSource<R: io::Read + io::Seek> {
    pub cameras: Cameras,
    pub images: Images,
    pub image_files: ImageFiles<R>,
    pub points: Points,
}
