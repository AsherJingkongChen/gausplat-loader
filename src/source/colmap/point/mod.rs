pub mod points;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use points::*;

use crate::function::{advance, read_any, write_any};
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub position: [f64; 3],
    pub color_rgb: [u8; 3],
}

impl Point {
    pub fn color_rgb_normalized(&self) -> [f32; 3] {
        [
            self.color_rgb[0] as f32 / 255.0,
            self.color_rgb[1] as f32 / 255.0,
            self.color_rgb[2] as f32 / 255.0,
        ]
    }
}

impl Decoder for Point {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let position = read_any::<[f64; 3]>(reader)?;
        let color_rgb = read_any::<[u8; 3]>(reader)?;
        // Skip re-projection error
        advance(reader, 8)?;
        let track_count = read_any::<u64>(reader)? as usize;
        // Skip tracks
        advance(reader, 8 * track_count)?;

        Ok(Self {
            position,
            color_rgb,
        })
    }
}

impl Encoder for Point {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        write_any(writer, &self.position)?;
        write_any(writer, &self.color_rgb)?;
        // Write -1.0 to re-projection error
        write_any(writer, &-1.0_f64)?;
        // Write 0 to track count
        write_any(writer, &0_u64)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn color_rgb_normalized() {
        use super::*;

        let source = Point {
            position: Default::default(),
            color_rgb: [255, 128, 0],
        };

        let target = [1.0, 0.5019608, 0.0];
        let output = source.color_rgb_normalized();
        assert_eq!(output, target);
    }
}
