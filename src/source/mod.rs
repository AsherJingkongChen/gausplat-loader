pub mod colmap;

pub use colmap::ColmapSource;
use std::collections::HashMap;
use crate::error::*;

pub trait Source {
    fn read_points(&mut self) -> Result<Vec<Point>, Error>;
    fn read_views(&mut self) -> Result<HashMap<u32, View>, Error>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub color: [u8; 3],
    pub position: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub affine_transformation: [[f64; 4]; 4],
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image_buffer: image::RgbImage,
    pub image_file_name: String,
    view_id: u32,
    pub view_height: u64,
    pub view_width: u64,
}

impl View {
    pub fn view_id(&self) -> &u32 {
        &self.view_id
    }
}
