#[derive(Clone, Debug, PartialEq)]
pub struct PinholeCamera {
    pub(crate) camera_id: u32,
    pub width: u64,
    pub height: u64,
    pub focal_length_x: f64,
    pub focal_length_y: f64,
}

impl PinholeCamera {
    pub fn projection_transform(&self) -> [[f64; 4]; 4] {
        const Z_FAR: f64 = 100.0;
        const Z_NEAR: f64 = 0.01;
        const Z_SIGN: f64 = 1.0;

        let x_tan_inv = (2.0 * self.focal_length_x) / (self.width as f64);
        let y_tan_inv = (2.0 * self.focal_length_y) / (self.height as f64);
        let z_scale = Z_FAR / (Z_FAR - Z_NEAR);

        [
            [x_tan_inv, 0.0, 0.0, 0.0],
            [0.0, y_tan_inv, 0.0, 0.0],
            [0.0, 0.0, Z_SIGN * z_scale, -Z_NEAR * z_scale],
            [0.0, 0.0, Z_SIGN, 0.0],
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn projection_transform_from_field_of_views_test() {
        use super::*;

        let camera = PinholeCamera {
            camera_id: 1,
            width: 1959,
            height: 1090,
            focal_length_x: 1159.5880733038061,
            focal_length_y: 1164.6601287484507,
        };
        let projection_transform = camera.projection_transform();
        assert_eq!(
            projection_transform,
            [
                [1.1838571447716244, 0.0, 0.0, 0.0],
                [0.0, 2.1369910619237626, 0.0, 0.0],
                [0.0, 0.0, 1.0001000100010002, -0.010001000100010003],
                [0.0, 0.0, 1.0, 0.0],
            ]
        );
    }
}
