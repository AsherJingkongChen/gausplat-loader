#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Unknown camera model id: {0}")]
    UnknownCameraModelId(u32),

    #[error("UTF-8 Error: {}", ._0.custom_display())]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub trait CustomDisplay {
    fn custom_display(&self) -> String;
}

impl CustomDisplay for std::string::FromUtf8Error {
    fn custom_display(&self) -> String {
        let utf8_error = self.utf8_error();
        let invalid_range = utf8_error.valid_up_to()
            ..(utf8_error.valid_up_to() + utf8_error.error_len().unwrap_or(0));
        let bytes = self.as_bytes();

        format!("{}: {:02x?}", self, &bytes[invalid_range])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn utf8_error() {
        use super::*;

        let error = String::from_utf8(vec![
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,
        ]);
        assert!(error.is_err(), "{:?}", error.unwrap());

        let text = error.unwrap_err().custom_display();
        assert!(text.contains("89"));
    }
}
