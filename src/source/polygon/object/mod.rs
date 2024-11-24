pub mod decode;
pub mod encode;

pub use super::*;

use derive_more::derive::{AsRef, Constructor, Display, From};
use Error::*;

#[derive(AsRef, Clone, Constructor, Debug, Display, Eq, From, PartialEq)]
#[display("{header}----------\n{payload}")]
pub struct Object {
    pub header: Header,
    pub payload: Payload,
}
