use crate::colmap::camera::Camera;

#[derive(Debug)]
pub enum DecodeError {
    Cast(bytemuck::checked::CheckedCastError),
    Io(std::io::Error),

    UnknownCameraModelId(u32),
    UnsupportedCameraModelType(Camera),
}
