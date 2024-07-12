#[derive(Debug)]
pub enum DecodeError {
    Io(std::io::Error),
    CastError(bytemuck::PodCastError),
    InvalidCameraModelId(u32),
}
