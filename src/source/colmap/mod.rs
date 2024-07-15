pub mod camera;
pub mod image;
pub mod image_file;
pub mod point;

use crate::error::*;
use crate::function::rotation_matrix_from_quaternion;
use crate::{function::field_of_view_from_focal_length, source};
pub use camera::*;
pub use image::*;
pub use image_file::*;
pub use point::*;
use std::collections::HashMap;
use std::io;

pub struct ColmapSource<R: io::Read + io::Seek> {
    pub cameras: Cameras,
    pub images: Images,
    pub image_files: ImageFiles<R>,
    pub points: Points,
}

impl<R: io::Read + io::Seek> source::Source for ColmapSource<R> {
    fn read_points(&mut self) -> Result<Vec<source::Point>, Error> {
        Ok(self
            .points
            .iter()
            .map(|point| source::Point {
                color: point.color.to_owned(),
                position: point.position.to_owned(),
            })
            .collect())
    }

    fn read_views(&mut self) -> Result<HashMap<u32, source::View>, Error> {
        let mut views = HashMap::with_capacity(self.images.len());
        for image in self.images.values() {
            let camera = {
                let key = image.camera_id();
                let value = self.cameras.get(key);
                if value.is_none() {
                    return Err(Error::NoSuchCameraId(key.to_owned()));
                }
                value.unwrap()
            };
            let image_file = {
                let key = image.file_name();
                let value = self.image_files.get_mut(key);
                if value.is_none() {
                    return Err(Error::NoSuchImageFileName(key.to_owned()));
                }
                value.unwrap()
            };

            let affine_transformation = {
                let r = rotation_matrix_from_quaternion(&image.rotation);
                let t = image.translation;
                [
                    [r[0][0], r[0][1], r[0][2], t[0]],
                    [r[1][0], r[1][1], r[1][2], t[1]],
                    [r[2][0], r[2][1], r[2][2], t[2]],
                    [0.0, 0.0, 0.0, 1.0],
                ]
            };
            let field_of_view_x = match camera {
                Camera::Pinhole(camera) => field_of_view_from_focal_length(
                    camera.focal_length_x,
                    camera.width as f64,
                ),
                _ => unimplemented!(),
            };
            let field_of_view_y = match camera {
                Camera::Pinhole(camera) => field_of_view_from_focal_length(
                    camera.focal_length_y,
                    camera.height as f64,
                ),
                _ => unimplemented!(),
            };
            let image_buffer = image_file.read()?;
            let image_file_name = image_file.file_name().to_owned();
            let view_height = camera.height().to_owned();
            let view_width = camera.width().to_owned();
            let view_id = image.image_id().to_owned();

            views.insert(
                view_id,
                source::View {
                    affine_transformation,
                    field_of_view_x,
                    field_of_view_y,
                    image_buffer,
                    image_file_name,
                    view_height,
                    view_id,
                    view_width,
                },
            );
        }

        Ok(views)
    }
}
