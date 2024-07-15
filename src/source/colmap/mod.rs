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
pub use source::Source;
use std::fmt;
use std::io;

#[derive(Clone, PartialEq)]
pub struct ColmapSource<R: io::Read + io::Seek> {
    pub cameras: Cameras,
    pub images: Images,
    pub image_files: ImageFiles<R>,
    pub points: Points,
}

impl<R: io::Read + io::Seek> Source for ColmapSource<R> {
    fn read_points(&mut self) -> Result<source::Points, Error> {
        Ok(self
            .points
            .iter()
            .map(|Point { color, position }| source::Point {
                color: [
                    color[0] as f64 / 255.0,
                    color[1] as f64 / 255.0,
                    color[2] as f64 / 255.0,
                ],
                position: position.to_owned(),
            })
            .collect())
    }

    fn read_views(&mut self) -> Result<source::Views, Error> {
        let duration = std::time::Instant::now();

        let views = self
            .images
            .values()
            .map(|image| {
                let camera = {
                    let key = image.camera_id();
                    let value = self.cameras.get(key);
                    if value.is_none() {
                        return Err(Error::NoSuchCameraId(key.to_owned()));
                    }
                    value.unwrap()
                };
                let image_file_name = image.file_name().to_owned();
                let image_file = {
                    let value = self.image_files.get_mut(&image_file_name);
                    if value.is_none() {
                        return Err(Error::NoSuchImageFileName(
                            image_file_name,
                        ));
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
                    _ => return Err(Error::Unimplemented),
                };
                let field_of_view_y = match camera {
                    Camera::Pinhole(camera) => field_of_view_from_focal_length(
                        camera.focal_length_y,
                        camera.height as f64,
                    ),
                    _ => return Err(Error::Unimplemented),
                };
                let view_id = image.image_id().to_owned();
                let view_height = camera.height().to_owned();
                let view_width = camera.width().to_owned();
                let image = image_file.read()?;

                let view = source::View {
                    affine_transformation,
                    field_of_view_x,
                    field_of_view_y,
                    image,
                    image_file_name,
                    view_id,
                    view_height,
                    view_width,
                };
                Ok((view_id, view))
            })
            .collect::<Result<_, Error>>();

        println!("Duration image file read: {:?}", duration.elapsed());

        views
    }
}

impl<R: io::Read + io::Seek> fmt::Debug for ColmapSource<R> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ColmapSource")
            .field("cameras.len()", &self.cameras.len())
            .field("images.len()", &self.images.len())
            .field("image_files.len()", &self.image_files.len())
            .field("points.len()", &self.points.len())
            .finish()
    }
}
