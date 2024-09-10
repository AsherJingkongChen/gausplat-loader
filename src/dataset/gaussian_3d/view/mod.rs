pub mod views;

pub use views::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct View {
    pub field_of_view_x: f64,
    pub field_of_view_y: f64,
    pub image_height: u32,
    pub image_width: u32,
    pub view_id: u32,

    /// The view position in world space.
    pub view_position: [f64; 3],

    /// The affine transformation matrix mapping
    /// from world space to view space **(in column-major order)**.
    ///
    /// ## Details
    ///
    /// ```ignore
    /// Tr_v = [R_v   | T_v]
    ///        [0 0 0 | 1  ]
    /// ```
    pub view_transform: [[f64; 4]; 4],
}

impl View {
    /// ## Arguments
    ///
    /// * `rotation_to_view` - A 3D rotation matrix mapping
    ///   from world space to view space **(in column-major order)**.
    ///
    /// * `translation_to_view` - A 3D translation vector mapping
    ///   from world space to view space.
    ///
    /// ## Returns
    ///
    /// A 3D view position in world space **(in column-major order)**.
    ///
    /// ## Details
    ///
    /// ```ignore
    /// // P_w is the view position in world space.
    /// // P_v is the view position in view space, which is the origin.
    /// // R_v is the rotation matrix mapping from world space to view space.
    /// // T_v is the translation vector mapping from world space to view space.
    ///
    /// P_v = 0 = R_v * P_w + T_v
    /// P_w = -R_v^t * T_v
    /// ```
    pub fn position(
        rotation_to_view: &[[f64; 3]; 3],
        translation_to_view: &[f64; 3],
    ) -> [f64; 3] {
        let r = rotation_to_view;
        let t = translation_to_view;
        [
            -r[0][0] * t[0] - r[0][1] * t[1] - r[0][2] * t[2],
            -r[1][0] * t[0] - r[1][1] * t[1] - r[1][2] * t[2],
            -r[2][0] * t[0] - r[2][1] * t[1] - r[2][2] * t[2],
        ]
    }

    /// ## Arguments
    ///
    /// * `quaternion_normalized` - A normalized quaternion `(x, y, z, w)`.
    ///
    /// ## Returns
    ///
    /// A 3D rotation matrix **(in column-major order)**.
    pub fn rotation(quaternion_normalized: &[f64; 4]) -> [[f64; 3]; 3] {
        let [x, y, z, w] = quaternion_normalized;
        let y_y = y * y * 2.0;
        let z_z = z * z * 2.0;
        let w_w = w * w * 2.0;
        let x_y = x * y * 2.0;
        let x_z = x * z * 2.0;
        let x_w = x * w * 2.0;
        let y_z = y * z * 2.0;
        let y_w = y * w * 2.0;
        let z_w = z * w * 2.0;
        [
            [1.0 - z_z - w_w, y_z + x_w, y_w - x_z],
            [y_z - x_w, 1.0 - y_y - w_w, z_w + x_y],
            [y_w + x_z, z_w - x_y, 1.0 - y_y - z_z],
        ]
    }

    /// ## Arguments
    ///
    /// * `rotation_to_view` - A 3D rotation matrix mapping
    ///   from world space to view space **(in column-major order)**.
    ///
    /// * `translation_to_view` - A 3D translation vector mapping
    ///   from world space to view space.
    ///
    /// ## Returns
    ///
    /// A 3D affine transformation matrix mapping
    ///   from world space to view space **(in column-major order)**.
    ///
    /// ## Details
    ///
    /// ```ignore
    /// Tr_v = [R_v   | T_v]
    ///        [0 0 0 | 1  ]
    /// ```
    pub fn transform_to_view(
        rotation_to_view: &[[f64; 3]; 3],
        translation_to_view: &[f64; 3],
    ) -> [[f64; 4]; 4] {
        let r = rotation_to_view;
        let t = translation_to_view;
        [
            [r[0][0], r[0][1], r[0][2], 0.0],
            [r[1][0], r[1][1], r[1][2], 0.0],
            [r[2][0], r[2][1], r[2][2], 0.0],
            [t[0], t[1], t[2], 1.0],
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn view_position() {
        use super::*;

        let quaternion_normalized = [
            0.9928923624805012,
            0.006208227229002722,
            -0.11837120574960786,
            0.010699163142319695,
        ];
        let translation_to_view =
            [2.1400970808418642, 0.18616441825409558, 4.726341984431894];

        let view_position = View::position(
            &View::rotation(&quaternion_normalized),
            &translation_to_view,
        );
        assert_eq!(
            view_position,
            [-3.194916373379071, -0.18378876753171225, -4.087996124741175]
        );
    }

    #[test]
    fn view_transform() {
        use super::*;

        let quaternion_normalized = [
            0.9961499472928047,
            -0.03510862409346388,
            -0.08026977784966388,
            0.003070795788047984,
        ];
        let translation_to_view = [0.129242027423, 0.0, -0.3424233862];

        let view_transform = View::transform_to_view(
            &View::rotation(&quaternion_normalized),
            &translation_to_view,
        );
        assert_eq!(
            view_transform,
            [
                [
                    0.9870946659543874,
                    0.011754269038001336,
                    0.1597058471183149,
                    0.0000000000000000,
                ],
                [
                    -0.000481623211642526,
                    0.9975159094549839,
                    -0.07043989227191047,
                    0.0000000000000000,
                ],
                [
                    -0.1601370927782764,
                    0.0694539238889973,
                    0.9846482945564589,
                    0.0000000000000000,
                ],
                [
                    0.129242027423,
                    0.0000000000000000,
                    -0.3424233862,
                    1.0000000000000000,
                ],
            ]
        );
    }
}
