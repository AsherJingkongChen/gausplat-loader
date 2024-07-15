use crate::source::colmap::Camera;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Cast(bytemuck::checked::CheckedCastError),
    Io(std::io::Error),
    Image(image::ImageError),
    NoSuchCameraId(u32),
    NoSuchImageFileName(String),
    UnknownCameraModelId(u32),
    Unimplemented,
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
