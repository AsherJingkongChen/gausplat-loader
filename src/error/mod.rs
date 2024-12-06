//! Error module.

/// Error variants.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error from the `bytemuck` crate.
    #[error("Bytemuck error: {0}")]
    Bytemuck(#[from] bytemuck::PodCastError),

    /// Error from the `globset` crate.
    #[error("Glob error: {0}")]
    Glob(#[from] globset::Error),

    /// Error from the `image` crate.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error from the `image` crate.
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    /// Error from the invalid ASCII string.
    #[error("Invalid ASCII string: {0:?}.")]
    InvalidAscii(String),

    /// Error from the invalid camera model id.
    #[error("Invalid camera model id: \"{0}\".")]
    InvalidCameraModelId(u32),

    /// Error from the invalid camera model name.
    #[error("Invalid kind: {0:?}.")]
    InvalidKind(String),

    /// Error from the invalid UTF-8 string.
    #[error("Invalid UTF-8 string: {0:?}.")]
    InvalidUtf8(String),

    /// Error from the mismatched [tensor shape](burn_tensor::Shape).
    #[error("Mismatched tensor shape: {0:?}. It should be {1:?}.")]
    MismatchedTensorShape(Vec<usize>, Vec<usize>),

    /// Error from the missing symbol.
    ///
    /// It generally comes from the decoding process.
    #[error("Missing symbol: {0:?}.")]
    MissingSymbol(String),

    /// Error from the out of bounds.
    #[error("Out of bounds: {0} is out of {1} at {2}.")]
    OutOfBounds(usize, usize, String),

    /// Error from the [`std::num::ParseIntError`].
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
