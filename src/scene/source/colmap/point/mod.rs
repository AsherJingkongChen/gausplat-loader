pub mod points;

pub use crate::function::Decoder;
pub use points::*;

use crate::{
    error::*,
    function::{advance, read_slice},
};
use std::io;

#[derive(Clone, Debug, PartialEq)]
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
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        advance(reader, 8)?;
        let position = read_slice!(reader, f64, 3)?;
        let color_rgb = read_slice!(reader, u8, 3)?;
        advance(reader, 8)?;
        {
            let track_count = read_slice!(reader, u64, 1)?[0] as usize;
            advance(reader, 8 * track_count)?;
        }

        Ok(Self {
            position,
            color_rgb,
        })
    }
}
