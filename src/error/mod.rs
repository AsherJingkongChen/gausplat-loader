#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Glob error: {0}")]
    Glob(#[from] globset::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Invalid ASCII string: {0:?}")]
    InvalidAscii(String),

    #[error("Invalid polygon property kind: {0:?}")]
    InvalidPolygonPropertyKind(String),

    #[error("Invalid UTF-8 string: {0:?}")]
    InvalidUtf8(String),

    #[error("Mismatched tensor shape: {0:?}. It should be {1:?}.")]
    MismatchedTensorShape(Vec<usize>, Vec<usize>),

    #[error("Missing token: {0:?}")]
    MissingToken(String),

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Unknown camera model id: {0}")]
    UnknownCameraModelId(u32),

    #[error("Unknown polygon format variant: {0:?}")]
    UnknownPolygonFormatVariant(String),

    #[error("Unknown polygon property kind: {0:?}")]
    UnknownPolygonPropertyKind(String),
}
