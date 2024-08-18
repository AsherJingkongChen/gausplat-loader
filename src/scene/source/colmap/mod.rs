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
use std::{fmt, io};

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

        let duration = std::time::Instant::now();
        let images_encoded = Vec::from_iter(source.image_files.into_values())
            .into_par_iter()
            .map(|mut image_file| {
                let image_encoded = image_file.read()?;
                Ok((image_file.file_name, image_encoded))
            })
            .collect::<Result<dashmap::DashMap<_, _>, Self::Error>>()?;
        println!("Duration (images_encoded): {:?}", duration.elapsed());

        let duration = std::time::Instant::now();
        let (images, views) = Vec::from_iter(source.images.into_values())
            .into_par_iter()
            .map(|image| {
                let view_position = image.view_position();
                let view_transform = image.view_transform();
                let camera_id = image.camera_id;
                let image_file_name = image.file_name;
                let view_id = image.image_id;

                let camera = source
                    .cameras
                    .get(&camera_id)
                    .ok_or(Error::UnknownCameraId(camera_id))?;
                let (field_of_view_x, field_of_view_y) = match camera {
                    Camera::Pinhole(camera) => (
                        2.0 * (camera.width as f64)
                            .atan2(2.0 * camera.focal_length_x),
                        2.0 * (camera.height as f64)
                            .atan2(2.0 * camera.focal_length_y),
                    ),
                };

                // Using Dashmap::remove since an image and a view
                // should have a 1-to-1 relationship
                let image_encoded = images_encoded
                    .remove(&image_file_name)
                    .ok_or(Error::UnknownImageFileName(
                        image_file_name.to_owned(),
                    ))?
                    .1;
                let image = sparse_view::Image {
                    image_encoded,
                    view_id,
                };
                let (image_width, image_height) = image.decode()?.dimensions();
                let view = sparse_view::View {
                    field_of_view_x,
                    field_of_view_y,
                    image_file_name,
                    image_height,
                    image_width,
                    view_id,
                    view_position,
                    view_transform,
                };

                Ok(((view_id, image), (view_id, view)))
            })
            .collect::<Result<_, Self::Error>>()?;
        println!("Duration (images, views): {:?}", duration.elapsed());

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
