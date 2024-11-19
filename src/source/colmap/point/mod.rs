pub mod points;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use points::*;

use crate::function::{advance, read_bytes_const};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{BufReader, BufWriter, Read, Write};

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
        let position = [
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
            reader.read_f64::<LE>()?,
        ];
        let color_rgb = read_bytes_const(reader)?;

        // Skip re-projection error
        advance(reader, 8)?;

        // Skip tracks
        let track_count = reader.read_u64::<LE>()? as usize;
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
        writer.write_f64::<LE>(self.position[0])?;
        writer.write_f64::<LE>(self.position[1])?;
        writer.write_f64::<LE>(self.position[2])?;
        writer.write_all(&self.color_rgb)?;

        // Write -1.0 to re-projection error
        writer.write_f64::<LE>(-1.0)?;

        // Write 0 to track count
        writer.write_u64::<LE>(0)?;

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
