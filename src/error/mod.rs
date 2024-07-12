#[derive(Debug)]
pub enum DecodeError {
    Cast(bytemuck::checked::CheckedCastError),
    Io(std::io::Error),

    InvalidCameraModelId(u32),
}
