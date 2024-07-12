use crate::colmap::camera::Camera;
use std::{error, fmt};

#[derive(Debug)]
pub enum DecodeError {
    Cast(bytemuck::checked::CheckedCastError),
    Io(std::io::Error),
    UnknownCameraModelId(u32),
    UnsupportedCameraModelType(Camera),
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for DecodeError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for DecodeError {}
