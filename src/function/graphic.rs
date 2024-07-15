pub fn field_of_view_from_focal_length(
    focal_length: f64,
    sensor_size: f64,
) -> f64 {
    2.0 * sensor_size.atan2(2.0 * focal_length)
}

pub fn projection_transform_from_field_of_views(
    field_of_view_x: f64,
    field_of_view_y: f64,
    z_far: f64,
    z_near: f64,
    z_sign: f64,
) -> [[f64; 4]; 4] {
    let fx_2_tan = (field_of_view_x / 2.0).tan();
    let fy_2_tan = (field_of_view_y / 2.0).tan();
    let x_max = fx_2_tan * z_near;
    let y_max = fy_2_tan * z_near;
    let z_scale = z_far / (z_far - z_near);

    [
        [z_near / x_max, 0.0, 0.0, 0.0],
        [0.0, z_near / y_max, 0.0, 0.0],
        [0.0, 0.0, z_sign * z_scale, -z_near * z_scale],
        [0.0, 0.0, z_sign, 0.0],
    ]
}
