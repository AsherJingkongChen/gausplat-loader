use crate::error::*;
use std::io;

pub trait Decoder
where
    Self: Sized,
{
    fn decode<R: io::Read>(reader: &mut R) -> Result<Self, DecodeError>;
}
