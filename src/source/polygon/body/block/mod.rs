pub mod packed;
pub mod planar;

pub use super::*;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use ascii::{AsAsciiStr, AsciiString, IntoAsciiString};

pub use packed::*;
pub use planar::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyBlock {
    pub id: Id,
    pub variant: BodyBlockVariant,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BodyBlockVariant {
    Packed(PackedBlock),
    Planar(PlanarBlock),
}
