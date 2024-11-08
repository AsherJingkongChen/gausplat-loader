#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PinholeCamera {
    pub camera_id: u32,
    pub width: u64,
    pub height: u64,
    pub focal_length_x: f64,
    pub focal_length_y: f64,
    pub principal_point_x: f64,
    pub principal_point_y: f64,
}
