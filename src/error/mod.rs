use crate::source::polygon::head::{FormatVariant, Head};

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

    #[error("Invalid camera model id: {0}")]
    InvalidCameraModelId(u32),

    // TODO: try this
    #[error(
        "Invalid polygon keyword: {0:?}. It should be one of {:?}.",
        Head::KEYWORD_DOMAIN
    )]
    InvalidPolygonKeyword(String),

    #[error(
        "Invalid polygon format variant: {0:?}. It should be one of {:?}.",
        FormatVariant::DOMAIN
    )]
    InvalidPolygonFormatVariant(String),

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
}
