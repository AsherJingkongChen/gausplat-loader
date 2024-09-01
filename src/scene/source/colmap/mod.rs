pub mod camera;
pub mod image;
pub mod point;

pub use super::file::*;
pub use camera::*;
pub use image::*;
pub use point::*;

use crate::scene::data::gaussian_3d;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fmt, io::Read};

pub struct ColmapSource<R> {
    pub cameras: Cameras,
    pub files: Files<R>,
    pub images: Images,
    pub points: Points,
}

impl<R: Read + Send> TryFrom<ColmapSource<R>> for gaussian_3d::Gaussian3dSceneData {
    type Error = Error;

    fn try_from(source: ColmapSource<R>) -> Result<Self, Self::Error> {
        let points = source
            .points
            .into_iter()
            .map(|point| gaussian_3d::Point {
                color_rgb: point.color_rgb_normalized(),
                position: point.position,
            })
            .collect();

        let images_encoded = Vec::from_iter(source.files.into_values())
            .into_par_iter()
            .map(|mut image_file| {
                let image_encoded = image_file.read()?;
                Ok((image_file.name, image_encoded))
            })
            .collect::<Result<dashmap::DashMap<_, _>, Self::Error>>()?;

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

                // Using ::remove since
                // an image and a view should have a 1-to-1 relationship
                let image_encoded = images_encoded
                    .remove(&image_file_name)
                    .ok_or(Error::UnknownImageFileName(
                        image_file_name.to_owned(),
                    ))?
                    .1;
                let image = gaussian_3d::Image {
                    image_encoded,
                    view_id,
                };
                let (image_width, image_height) =
                    image.decode_rgb()?.dimensions();
                let view = gaussian_3d::View {
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

        #[cfg(debug_assertions)]
        log::debug!(
            target: "gausplat_importer::scene",
            "Gaussian3dSceneData::try_from(ColmapSource)",
        );

        Ok(Self {
            images,
            points,
            views,
        })
    }
}

impl<R> fmt::Debug for ColmapSource<R> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ColmapSource")
            .field("cameras.len()", &self.cameras.len())
            .field("files.len()", &self.files.len())
            .field("images.len()", &self.images.len())
            .field("points.len()", &self.points.len())
            .finish()
    }
}
