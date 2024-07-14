pub fn field_of_view_from_focal_length(
    focal_length: f64,
    sensor_size: f64,
) -> f64 {
    2.0 * sensor_size.atan2(2.0 * focal_length)
}
