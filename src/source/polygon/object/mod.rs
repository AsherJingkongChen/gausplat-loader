pub mod access;
pub mod decode;
pub mod encode;

pub use super::*;
pub use access::*;
pub use bytemuck::Pod;
pub use header::*;

use derive_more::derive::{AsRef, Constructor, Display, From};
use Error::*;

#[derive(AsRef, Clone, Constructor, Debug, Default, Display, Eq, From, PartialEq)]
#[display("{header}----------\n{payload}")]
pub struct Object {
    pub header: Header,
    pub payload: Payload,
}
