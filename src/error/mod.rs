pub use crate::scene::colmap::Camera;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Unknown camera id: {0}")]
    UnknownCameraId(u32),

    #[error("Unknown camera model id: {0}")]
    UnknownCameraModelId(u32),

    #[error("Unknown image file name: {0}")]
    UnknownImageFileName(String),

    #[error("UTF-8 Error: {}", ._0.format())]
    Utf8(#[from] std::string::FromUtf8Error),
}

trait ErrorDisplay {
    fn format(&self) -> String;
}

impl ErrorDisplay for std::string::FromUtf8Error {
    fn format(&self) -> String {
        let utf8_error = self.utf8_error();
        let invalid_range = utf8_error.valid_up_to()
            ..(utf8_error.valid_up_to() + utf8_error.error_len().unwrap_or(0));
        let bytes = self.as_bytes();

        format!("{}: {:02x?}", self, &bytes[invalid_range])
    }
}
