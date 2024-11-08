pub mod points;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use points::*;

use crate::function::{advance, read_any};
use std::io::Read;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub position: [f64; 3],
    pub color_rgb: [u8; 3],
}

impl Point {
    pub fn color_rgb_normalized(&self) -> [f64; 3] {
        [
            self.color_rgb[0] as f64 / 255.0,
            self.color_rgb[1] as f64 / 255.0,
            self.color_rgb[2] as f64 / 255.0,
        ]
    }
}

impl Decoder for Point {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        advance(reader, 8)?;
        let position = read_any::<[f64; 3]>(reader)?;
        let color_rgb = read_any::<[u8; 3]>(reader)?;
        advance(reader, 8)?;
        let track_count = read_any::<u64>(reader)? as usize;
        advance(reader, 8 * track_count)?;

        Ok(Self {
            position,
            color_rgb,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn color_rgb_normalized() {
        use super::*;

        let point = Point {
            position: [0.0, 0.0, 0.0],
            color_rgb: [255, 0, 0],
        };

        let color_rgb_normalized = point.color_rgb_normalized();
        assert_eq!(color_rgb_normalized, [1.0, 0.0, 0.0]);
    }
}
