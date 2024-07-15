#[derive(Debug, PartialEq)]
pub struct View {
    pub affine_transformation: [[f64; 4]; 4],
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image: image::RgbImage,
    pub(super) image_file_name: String,
    pub(super) view_id: u32,
    pub view_height: u64,
    pub view_width: u64,
}

impl View {
    pub fn image_file_name(&self) -> &str {
        &self.image_file_name
    }

    pub fn view_id(&self) -> &u32 {
        &self.view_id
    }
}
