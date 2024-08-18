pub mod points;

pub use crate::error::Error;
pub use crate::function::Decoder;
pub use points::*;

use crate::function::{advance, read_slice};
use bytemuck::{Pod, Zeroable};
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
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
        #[derive(Clone, Copy, Pod, Zeroable)]
        #[repr(C)]
        struct Packet(f64, [f64; 3]);

        let [Packet(_, position)] = read_slice::<Packet, 1>(reader)?;
        let color_rgb = read_slice::<u8, 3>(reader)?;
        advance(reader, 8)?;
        let track_count = read_slice::<u64, 1>(reader)?[0] as usize;
        advance(reader, 8 * track_count)?;

        Ok(Self {
            position,
            color_rgb,
        })
    }
}
