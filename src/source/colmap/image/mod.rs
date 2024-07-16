pub mod images;

pub use crate::function::Decoder;
use crate::{
    error::*,
    function::{advance, read_slice},
};
pub use images::*;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    image_id: u32,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    camera_id: u32,
    file_name: String,
}

impl Image {
    pub fn image_id(&self) -> &u32 {
        &self.image_id
    }

    pub fn camera_id(&self) -> &u32 {
        &self.camera_id
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn view_transform(&self) -> [[f64; 4]; 4] {
        let [r0, r1, r2, r3] = self.rotation;
        let [t0, t1, t2] = self.translation;
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
            [1.0 - r2_r2 - r3_r3, r1_r2 - r0_r3, r1_r3 + r0_r2, t0],
            [r1_r2 + r0_r3, 1.0 - r1_r1 - r3_r3, r2_r3 - r0_r1, t1],
            [r1_r3 - r0_r2, r2_r3 + r0_r1, 1.0 - r1_r1 - r2_r2, t2],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

impl Decoder for Image {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let [image_id] = read_slice!(reader, u32, 1)?;
        let rotation = read_slice!(reader, f64, 4)?;
        let translation = read_slice!(reader, f64, 3)?;
        let [camera_id] = read_slice!(reader, u32, 1)?;
        let file_name = {
            let mut bytes = Vec::new();
            loop {
                let byte = read_slice!(reader, u8, 1)?[0];
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }
            String::from_utf8(bytes).map_err(Error::Utf8)?
        };
        {
            let point_count = read_slice!(reader, u64, 1)?[0] as usize;
            advance(reader, 24 * point_count)?;
        };

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
    fn view_transform() {
        use super::*;

        let image = Image {
            image_id: 0,
            rotation: [
                0.9961499472928047,
                -0.03510862409346388,
                -0.08026977784966388,
                0.003070795788047984,
            ],
            translation: [0.129242027423, 0.0, -0.3424233862],
            camera_id: 0,
            file_name: String::new(),
        };

        let view_transform = image.view_transform();
        assert_eq!(
            view_transform,
            [
                [
                    0.9870946659543874,
                    -0.000481623211642526,
                    -0.1601370927782764,
                    0.129242027423,
                ],
                [
                    0.011754269038001336,
                    0.9975159094549839,
                    0.0694539238889973,
                    0.0
                ],
                [
                    0.1597058471183149,
                    -0.07043989227191047,
                    0.9846482945564589,
                    -0.3424233862
                ],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
    }
}
