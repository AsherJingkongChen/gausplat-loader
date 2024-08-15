pub use crate::scene::source::colmap::Camera;

use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Cast(bytemuck::checked::CheckedCastError),
    Io(std::io::Error),
    Image(image::ImageError),
    Unimplemented,
    UnknownCameraId(u32),
    UnknownCameraModelId(u32),
    UnknownImageFileName(String),
    UnsupportedCameraModel(Camera),
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}
