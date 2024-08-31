pub mod images;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use images::*;

use crate::function::{advance, read_slice};
use bytemuck::{Pod, Zeroable};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub image_id: u32,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    pub camera_id: u32,
    pub file_name: String,
}

impl Image {
    /// The transformation matrix computed from the normalized quaternion `self.rotation`
    pub fn rotation_transform(&self) -> [[f64; 3]; 3] {
        let [r0, r1, r2, r3] = self.rotation;
        let r1_r1 = r1 * r1 * 2.0;
        let r2_r2 = r2 * r2 * 2.0;
        let r3_r3 = r3 * r3 * 2.0;
        let r0_r1 = r0 * r1 * 2.0;
        let r0_r2 = r0 * r2 * 2.0;
        let r0_r3 = r0 * r3 * 2.0;
        let r1_r2 = r1 * r2 * 2.0;
        let r1_r3 = r1 * r3 * 2.0;
        let r2_r3 = r2 * r3 * 2.0;
        [
            [1.0 - r2_r2 - r3_r3, r1_r2 - r0_r3, r1_r3 + r0_r2],
            [r1_r2 + r0_r3, 1.0 - r1_r1 - r3_r3, r2_r3 - r0_r1],
            [r1_r3 - r0_r2, r2_r3 + r0_r1, 1.0 - r1_r1 - r2_r2],
        ]
    }

    /// The position of the camera in world space
    pub fn view_position(&self) -> [f64; 3] {
        let r = self.rotation_transform();
        let t = self.translation;
        [
            -r[0][0] * t[0] - r[1][0] * t[1] - r[2][0] * t[2],
            -r[0][1] * t[0] - r[1][1] * t[1] - r[2][1] * t[2],
            -r[0][2] * t[0] - r[1][2] * t[1] - r[2][2] * t[2],
        ]
    }

    /// The transformation matrix from world space to camera space in **column-major** order
    pub fn view_transform(&self) -> [[f64; 4]; 4] {
        let r = self.rotation_transform();
        let t = self.translation;
        [
            [r[0][0], r[1][0], r[2][0], 0.0],
            [r[0][1], r[1][1], r[2][1], 0.0],
            [r[0][2], r[1][2], r[2][2], 0.0],
            [t[0], t[1], t[2], 1.0],
        ]
    }
}

impl Decoder for Image {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, Error> {
        #[repr(C)]
        #[derive(Clone, Copy, Pod, Zeroable)]
        struct Packet([f64; 4], [f64; 3]);

        let [image_id] = read_slice::<u32, 1>(reader)?;
        let [Packet(rotation, translation)] = read_slice::<Packet, 1>(reader)?;
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
            rotation,
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
            rotation: [
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

        let view_position = image.view_position();
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
            rotation: [
                0.9961499472928047,
                -0.03510862409346388,
                -0.08026977784966388,
                0.003070795788047984,
            ],
            translation: [0.129242027423, 0.0, -0.3424233862],
            camera_id: Default::default(),
            file_name: Default::default(),
        };

        let view_transform = image.view_transform();
        assert_eq!(
            view_transform,
            [
                [
                    0.9870946659543874,
                    0.011754269038001336,
                    0.1597058471183149,
                    0.0
                ],
                [
                    -0.000481623211642526,
                    0.9975159094549839,
                    -0.07043989227191047,
                    0.0
                ],
                [
                    -0.1601370927782764,
                    0.0694539238889973,
                    0.9846482945564589,
                    0.0
                ],
                [0.129242027423, 0.0, -0.3424233862, 1.0],
            ]
        );
    }
}
