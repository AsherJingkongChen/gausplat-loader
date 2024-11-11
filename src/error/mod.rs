use std::ffi::OsString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Globset error: {0}")]
    Globset(#[from] globset::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Invalid UTF-8 string: {:?}", .0.to_string_lossy())]
    InvalidUtf8(OsString),

    #[error("Mismatched tensor shape: {0:?}. It should be {1:?}.")]
    MismatchedTensorShape(Vec<usize>, Vec<usize>),

    #[error("Unknown camera model id: {0}")]
    UnknownCameraModelId(u32),
}
