pub mod images;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use images::*;

use crate::function::{advance, read_slice};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub image_id: u32,
    pub quaternion: [f64; 4],
    pub translation: [f64; 3],
    pub camera_id: u32,
    pub file_name: String,
}

impl Image {
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
    /// from world space to view space **(in column-major order)**.
    ///
    /// * `translation_to_view` - A 3D translation vector mapping
    /// from world space to view space.
    ///
    /// ## Returns
    ///
    /// A 3D affine transformation matrix mapping
    /// from world space to view space **(in column-major order)**.
    ///
    /// ## Details
    ///
    /// ```ignore
    /// Tr_wv = [R_wv  | T_wv]
    ///         [0 0 0 | 1   ]
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

    /// ## Arguments
    ///
    /// * `rotation_to_view` - A 3D rotation matrix mapping
    /// from world space to view space **(in column-major order)**.
    ///
    /// * `translation_to_view` - A 3D translation vector mapping
    /// from world space to view space.
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
    pub fn view_position(
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
}

impl Decoder for Image {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let [image_id] = read_slice::<u32, 1>(reader)?;
        let quaternion = read_slice::<f64, 4>(reader)?;
        let translation = read_slice::<f64, 3>(reader)?;
        let [camera_id] = read_slice::<u32, 1>(reader)?;
        let file_name = {
            let mut bytes = Vec::with_capacity(16);
            loop {
                let [byte] = read_slice::<u8, 1>(reader)?;
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }
            String::from_utf8(bytes)?
        };
        let point_count = read_slice::<u64, 1>(reader)?[0] as usize;
        advance(reader, 24 * point_count)?;

        Ok(Self {
            image_id,
            quaternion,
            translation,
            camera_id,
            file_name,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn view_position() {
        use super::*;

        let image = Image {
            image_id: Default::default(),
            quaternion: [
                0.9928923624805012,
                0.006208227229002722,
                -0.11837120574960786,
                0.010699163142319695,
            ],
            translation: [
                2.1400970808418642,
                0.18616441825409558,
                4.726341984431894,
            ],
            camera_id: Default::default(),
            file_name: Default::default(),
        };

        let view_position = Image::view_position(
            &Image::rotation(&image.quaternion),
            &image.translation,
        );
        assert_eq!(
            view_position,
            [-3.194916373379071, -0.18378876753171225, -4.087996124741175]
        );
    }

    #[test]
    fn view_transform() {
        use super::*;

        let image = Image {
            image_id: Default::default(),
            quaternion: [
                0.9961499472928047,
                -0.03510862409346388,
                -0.08026977784966388,
                0.003070795788047984,
            ],
            translation: [0.129242027423, 0.0, -0.3424233862],
            camera_id: Default::default(),
            file_name: Default::default(),
        };

        let view_transform = Image::transform_to_view(
            &Image::rotation(&image.quaternion),
            &image.translation,
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
