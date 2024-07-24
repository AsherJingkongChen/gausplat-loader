pub mod camera;
pub mod image;
pub mod image_file;
pub mod point;

pub use camera::*;
pub use image::*;
pub use image_file::*;
pub use point::*;

use crate::scene::sparse_view;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::io;

pub struct ColmapSource<R: io::Read + io::Seek> {
    pub cameras: Cameras,
    pub images: Images,
    pub image_files: ImageFiles<R>,
    pub points: Points,
}

impl<R: io::Read + io::Seek + Send + Sync> TryFrom<ColmapSource<R>>
    for sparse_view::SparseViewScene
{
    type Error = Error;

    fn try_from(source: ColmapSource<R>) -> Result<Self, Self::Error> {
        let points = source
            .points
            .into_iter()
            .map(|point| sparse_view::Point {
                color_rgb: point.color_rgb_normalized(),
                position: point.position,
            })
            .collect();

        let (images, views) = source
            .images
            .into_par_iter()
            .map(|(_, image)| {
                let camera = {
                    let key = image.camera_id;
                    let value = source.cameras.get(&key);
                    if value.is_none() {
                        return Err(Error::UnknownCameraId(key));
                    }
                    value.unwrap()
                };
                let mut image_file = {
                    let value = source.image_files.get_mut(&image.file_name);
                    if value.is_none() {
                        return Err(Error::UnknownImageFileName(
                            image.file_name,
                        ));
                    }
                    value.unwrap()
                };

                let (focal_length_x, focal_length_y) = match camera {
                    Camera::Pinhole(camera) => {
                        (camera.focal_length_x, camera.focal_length_y)
                    },
                    _ => return Err(Error::Unimplemented),
                };
                let projection_transform = match camera {
                    Camera::Pinhole(camera) => camera.projection_transform(),
                    _ => return Err(Error::Unimplemented),
                };
                let view_id = image.image_id;
                let view_position = image.view_position();
                let view_transform = image.view_transform();
                let image_file_name = image.file_name;
                let image = image_file.read()?;
                let image_height = image.height();
                let image_width = image.width();

                let image = sparse_view::Image { image, view_id };
                let view = sparse_view::View {
                    focal_length_x,
                    focal_length_y,
                    image_file_name,
                    image_height,
                    image_width,
                    projection_transform,
                    view_id,
                    view_position,
                    view_transform,
                };
                Ok(((view_id, image), (view_id, view)))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            images,
            points,
            views,
        })
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
