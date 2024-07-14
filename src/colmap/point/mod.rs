pub mod points;

pub use crate::function::Decoder;
use crate::{
    error::*,
    function::{advance, read_to_slice},
};
pub use points::*;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub position: [f64; 3],
    pub color: [u8; 3],
}

impl Decoder for Point {
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
        advance(reader, 8)?;
        let position = read_to_slice!(reader, f64, 3)?;
        let color = read_to_slice!(reader, u8, 3)?;
        advance(reader, 8)?;
        {
            let track_count = read_to_slice!(reader, u64, 1)?[0] as usize;
            advance(reader, 8 * track_count)?;
        }

        Ok(Self { position, color })
    }
}
