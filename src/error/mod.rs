#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bytemuck error: {0}")]
    Bytemuck(#[from] bytemuck::PodCastError),

    #[error("Cast int error: {0}")]
    CastIntError(#[from] std::num::TryFromIntError),

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

    #[error("Invalid UTF-8 string: {0:?}")]
    InvalidUtf8(String),

    #[error("Misaligned bytes: size of {0} is not aligned to {1}.")]
    MisalignedBytes(usize, usize),

    #[error("Mismatched tensor shape: {0:?}. It should be {1:?}.")]
    MismatchedTensorShape(Vec<usize>, Vec<usize>),

    #[error("Missing token: {0:?}")]
    MissingToken(String),

    #[error("Out of bounds: {0} is out of {1}.")]
    OutOfBounds(usize, usize),

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
