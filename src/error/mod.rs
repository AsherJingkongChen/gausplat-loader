#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bytemuck error: {0}")]
    Bytemuck(#[from] bytemuck::PodCastError),

    #[error("Glob error: {0}")]
    Glob(#[from] globset::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Invalid ASCII string: {0:?}.")]
    InvalidAscii(String),

    #[error("Invalid camera model id: \"{0}\".")]
    InvalidCameraModelId(u32),

    #[error("Invalid kind: {0:?}.")]
    InvalidKind(String),

    #[error("Invalid UTF-8 string: {0:?}.")]
    InvalidUtf8(String),

    #[error("Mismatched tensor shape: {0:?}. It should be {1:?}.")]
    MismatchedTensorShape(Vec<usize>, Vec<usize>),

    #[error("Missing symbol: {0:?}.")]
    MissingSymbol(String),

    #[error("Out of bounds: {0} is out of {1}.")]
    OutOfBounds(usize, usize),

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
