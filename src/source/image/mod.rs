pub mod images;

pub use crate::error::Error;
pub use burn_tensor::{backend::Backend, Tensor};
pub use images::*;

use crate::image_crate::{ColorType, GenericImageView, ImageFormat};
use burn_tensor::TensorData;
use std::{fmt, io::Cursor};

#[derive(Clone, Default, PartialEq)]
pub struct Image {
    pub image_encoded: Vec<u8>,
    pub image_file_name: String,
    pub image_id: u32,
}

impl Image {
    pub fn decode_dimensions(&self) -> Result<(u32, u32), Error> {
        Ok(image::load_from_memory(&self.image_encoded)?.dimensions())
    }

    /// Decoding to a tensor with shape of `[H, W, C]`,
    /// where `C` is the channel count of RGB image.
    pub fn decode_rgb_to_tensor<B: Backend>(
        &self,
        device: &B::Device,
    ) -> Result<Tensor<B, 3>, Error> {
        const CHANNEL_COUNT: usize = 3;

        let image = image::load_from_memory(&self.image_encoded)?.into_rgb8();
        let (width, height) = image.dimensions();
        let value = TensorData::new(
            image.into_raw(),
            [height as usize, width as usize, CHANNEL_COUNT],
        );

        Ok(Tensor::from_data(value, device).div_scalar(255.0))
    }

    /// Encoding a tensor with shape of `[H, W, C]` to an image,
    /// where `C` is the channel count of RGB image.
    pub fn encode_rgb_from_tensor<B: Backend>(
        &mut self,
        tensor: Tensor<B, 3>,
    ) -> Result<&mut Self, Error> {
        let [height, width, channel_count] = tensor.dims();
        if channel_count != 3 {
            Err(Error::MismatchedTensorShape(
                vec![height, width, channel_count],
                vec![height, width, 3],
            ))?;
        }

        let mut result = Cursor::new(Vec::new());
        let value = tensor
            .mul_scalar(255.0)
            .add_scalar(0.5)
            .clamp(0.0, 255.0)
            .into_data()
            .convert::<u8>()
            .bytes;

        image::write_buffer_with_format(
            &mut result,
            &value,
            width as u32,
            height as u32,
            ColorType::Rgb8,
            ImageFormat::from_path(&self.image_file_name)?,
        )?;
        self.image_encoded = result.into_inner();

        Ok(self)
    }
}

impl fmt::Debug for Image {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Image")
            .field("image_encoded.len()", &self.image_encoded.len())
            .field("image_file_name", &self.image_file_name)
            .field("image_id", &self.image_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_rgb_to_tensor() {
        use super::*;
        use burn_ndarray::NdArray;

        let image = Image {
            image_encoded: vec![
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00,
                0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
                0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x01, 0x73, 0x52, 0x47,
                0x42, 0x00, 0xae, 0xce, 0x1c, 0xe9, 0x00, 0x00, 0x00, 0x44,
                0x65, 0x58, 0x49, 0x66, 0x4d, 0x4d, 0x00, 0x2a, 0x00, 0x00,
                0x00, 0x08, 0x00, 0x01, 0x87, 0x69, 0x00, 0x04, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x03, 0xa0, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x01, 0x00, 0x00, 0xa0, 0x02, 0x00, 0x04, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xa0, 0x03, 0x00, 0x04,
                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00, 0xf9, 0x22, 0x9d, 0xfe, 0x00, 0x00, 0x00, 0x0d,
                0x49, 0x44, 0x41, 0x54, 0x08, 0x1d, 0x63, 0xf8, 0xcf, 0x60,
                0xdb, 0x0d, 0x00, 0x05, 0x06, 0x01, 0xc8, 0x5d, 0xd6, 0x92,
                0xd1, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae,
                0x42, 0x60, 0x82,
            ],
            image_file_name: Default::default(),
            image_id: Default::default(),
        };

        assert_eq!(
            image
                .decode_rgb_to_tensor::<NdArray>(&Default::default())
                .unwrap()
                .into_data()
                .to_vec::<f32>()
                .unwrap(),
            vec![1.0, 0.0, 0.23921569]
        );
    }

    #[test]
    fn recode_tensor() {
        use super::*;
        use burn_ndarray::NdArray;
        use std::io::Cursor;

        let image_file_name = "example.png";
        let image_encoded_source = include_bytes!("example.png");

        let mut image_encoded_target = Cursor::new(Vec::new());
        image::load_from_memory(image_encoded_source)
            .unwrap()
            .write_to(
                &mut image_encoded_target,
                image::ImageFormat::from_path(image_file_name).unwrap(),
            )
            .unwrap();
        let image_encoded_target = &image_encoded_target.into_inner();

        let mut image = Image {
            image_encoded: image_encoded_target.to_owned(),
            image_file_name: image_file_name.into(),
            image_id: Default::default(),
        };

        // It should be idempotent
        for _ in 0..5 {
            let image_encoded_output = &image
                .encode_rgb_from_tensor(
                    image
                        .decode_rgb_to_tensor::<NdArray>(&Default::default())
                        .unwrap(),
                )
                .unwrap()
                .image_encoded;

            assert_eq!(image_encoded_output, image_encoded_target);
        }
    }
}
