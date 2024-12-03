pub mod images;

pub use crate::error::Error;
pub use burn_tensor::{backend::Backend, Tensor};
pub use image::RgbImage;
pub use images::*;

use super::file::{File, Opener};
use burn_tensor::TensorData;
use image::{GenericImageView, ImageFormat, Pixel, Rgb};
use std::{fmt, io::Cursor, path::PathBuf};

#[derive(Clone, Default, PartialEq)]
pub struct Image {
    pub image_encoded: Vec<u8>,
    pub image_file_path: PathBuf,
    pub image_id: u32,
}

/// Interoperability with [`RgbImage`].
impl Image {
    /// Decoding the image dimensions `(width, height)`.
    #[inline]
    pub fn decode_dimensions(&self) -> Result<(u32, u32), Error> {
        Ok(image::load_from_memory(&self.image_encoded)?.dimensions())
    }

    /// Decoding an [`RgbImage`] from [`Self::image_encoded`].
    #[inline]
    pub fn decode_rgb(&self) -> Result<RgbImage, Error> {
        Ok(image::load_from_memory(&self.image_encoded)?.into_rgb8())
    }

    /// Encoding an [`RgbImage`] to [`Self::image_encoded`].
    pub fn encode_rgb(
        &mut self,
        image: RgbImage,
    ) -> Result<&mut Self, Error> {
        const CHANNEL_COUNT: u32 = Rgb::<u8>::CHANNEL_COUNT as u32;

        let (width, height) = image.dimensions();
        let mut writer = Cursor::new(Vec::with_capacity(
            (height * width * CHANNEL_COUNT) as usize,
        ));
        image.write_to(&mut writer, ImageFormat::from_path(&self.image_file_path)?)?;
        self.image_encoded = writer.into_inner();

        Ok(self)
    }
}

/// Interoperability with [`Tensor`].
impl Image {
    /// Obtaining an [`RgbImage`] from a [`Tensor`] with shape of `[H, W, C]`
    /// where `C == 3`.
    pub fn get_rgb_from_tensor<B: Backend>(
        tensor: Tensor<B, 3>
    ) -> Result<RgbImage, Error> {
        const CHANNEL_COUNT: usize = Rgb::<u8>::CHANNEL_COUNT as usize;

        let [height, width, channel_count] = tensor.dims();
        if channel_count != CHANNEL_COUNT {
            return Err(Error::MismatchedTensorShape(
                vec![height, width, channel_count],
                vec![height, width, CHANNEL_COUNT],
            ));
        }

        // NOTE: The data type is converted.
        let value = tensor
            .mul_scalar(255.0)
            .add_scalar(0.5)
            .clamp(0.0, 255.0)
            .into_data()
            .convert::<u8>()
            .into_vec()
            .unwrap();

        // NOTE: The data size just fits.
        Ok(RgbImage::from_raw(width as u32, height as u32, value).unwrap())
    }

    /// Obtaining a [`Tensor`] with shape of `[H, W, C]` from an [`RgbImage`]
    /// where `C == 3`.
    #[inline]
    pub fn get_tensor_from_rgb<B: Backend>(
        image: RgbImage,
        device: &B::Device,
    ) -> Tensor<B, 3> {
        const CHANNEL_COUNT: usize = Rgb::<u8>::CHANNEL_COUNT as usize;

        let (width, height) = image.dimensions();
        let value = TensorData::new(
            image.into_raw(),
            [height as usize, width as usize, CHANNEL_COUNT],
        );

        Tensor::from_data(value, device).div_scalar(255.0)
    }

    /// Decoding an [`RgbImage`] from [`Self::image_encoded`],
    /// and converting it to a [`Tensor`].
    #[inline]
    pub fn decode_rgb_tensor<B: Backend>(
        &self,
        device: &B::Device,
    ) -> Result<Tensor<B, 3>, Error> {
        Ok(Self::get_tensor_from_rgb(self.decode_rgb()?, device))
    }

    /// Converting a [`Tensor`] with shape of `[H, W, C]` to an [`RgbImage`],
    /// and encoding it to [`Self::image_encoded`].
    #[inline]
    pub fn encode_rgb_tensor<B: Backend>(
        &mut self,
        tensor: Tensor<B, 3>,
    ) -> Result<&mut Self, Error> {
        self.encode_rgb(Self::get_rgb_from_tensor(tensor)?)
    }
}

impl Image {
    pub fn save(&self) -> Result<&Self, Error> {
        File::open(&self.image_file_path)?
            .truncate()?
            .write_all(&self.image_encoded)?;
        Ok(self)
    }
}

impl fmt::Debug for Image {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Image")
            .field("image_encoded.len()", &self.image_encoded.len())
            .field("image_file_path", &self.image_file_path)
            .field("image_id", &self.image_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn debug_and_default() {
        use super::*;

        let target = Image {
            image_encoded: Default::default(),
            image_file_path: Default::default(),
            image_id: Default::default(),
        };
        let output = Image::default();
        assert_eq!(output, target);

        let target = true;
        let output = format!("{:?}", Image::default()).starts_with("Image");
        assert_eq!(output, target);
    }

    #[test]
    fn decode_and_encode_rgb_between_tensor() {
        use super::*;
        use burn_ndarray::NdArray;

        let source = &include_bytes!("../../../examples/data/image/example.png")[..];
        let mut image = Image {
            image_encoded: source.to_vec(),
            image_file_path: "example.png".into(),
            image_id: Default::default(),
        };

        (0..5).for_each(|_| {
            let target = image
                .decode_rgb_tensor::<NdArray>(&Default::default())
                .unwrap();
            let output = image
                .encode_rgb_tensor(target.to_owned())
                .unwrap()
                .decode_rgb_tensor::<NdArray>(&Default::default())
                .unwrap();
            output.into_data().assert_eq(&target.into_data(), true);
        });
    }

    #[test]
    fn decode_dimensions() {
        use super::*;

        let source = &include_bytes!("../../../examples/data/image/example.png")[..];
        let image = Image {
            image_encoded: source.to_vec(),
            image_file_path: "example.png".into(),
            image_id: Default::default(),
        };

        let target = (172, 178);
        let output = image.decode_dimensions().unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_rgb_tensor() {
        use super::*;
        use burn_ndarray::NdArray;

        let source = &include_bytes!("../../../examples/data/image/rainbow-8x8.png")[..];
        let image = Image {
            image_encoded: source.to_vec(),
            image_file_path: "rainbow-8x8.png".into(),
            image_id: Default::default(),
        };

        let target = Tensor::<NdArray, 3>::from([
            [
                [1.00000000, 0.87843138, 0.87843138],
                [1.00000000, 0.96862745, 0.87843138],
                [0.94117647, 1.00000000, 0.87843138],
                [0.87843138, 1.00000000, 0.90588236],
                [0.87843138, 1.00000000, 1.00000000],
                [0.87843138, 0.90980393, 1.00000000],
                [0.93725491, 0.87843138, 1.00000000],
                [1.00000000, 0.87843138, 0.96862745],
            ],
            [
                [1.00000000, 0.75294119, 0.75294119],
                [1.00000000, 0.93333334, 0.75294119],
                [0.88235295, 1.00000000, 0.75294119],
                [0.75294119, 1.00000000, 0.81176472],
                [0.75294119, 1.00000000, 0.99607843],
                [0.75294119, 0.81568629, 1.00000000],
                [0.87450981, 0.75294119, 1.00000000],
                [1.00000000, 0.75294119, 0.93725491],
            ],
            [
                [1.00000000, 0.62745100, 0.62745100],
                [1.00000000, 0.89803922, 0.62745100],
                [0.81960785, 1.00000000, 0.62745100],
                [0.62745100, 1.00000000, 0.71372551],
                [0.62745100, 1.00000000, 0.99607843],
                [0.62745100, 0.72549021, 1.00000000],
                [0.81176472, 0.62745100, 1.00000000],
                [1.00000000, 0.62745100, 0.90980393],
            ],
            [
                [1.00000000, 0.50196081, 0.50196081],
                [1.00000000, 0.86666667, 0.50196081],
                [0.76078433, 1.00000000, 0.50196081],
                [0.50196081, 1.00000000, 0.61960787],
                [0.50196081, 1.00000000, 0.99607843],
                [0.50196081, 0.63137257, 1.00000000],
                [0.74901962, 0.50196081, 1.00000000],
                [1.00000000, 0.50196081, 0.87843138],
            ],
            [
                [1.00000000, 0.37647060, 0.37647060],
                [1.00000000, 0.83137256, 0.37647060],
                [0.69803923, 1.00000000, 0.37647060],
                [0.37647060, 1.00000000, 0.52156866],
                [0.37647060, 1.00000000, 0.99215686],
                [0.37647060, 0.53725493, 1.00000000],
                [0.68627453, 0.37647060, 1.00000000],
                [1.00000000, 0.37647060, 0.84705883],
            ],
            [
                [1.00000000, 0.25098041, 0.25098041],
                [1.00000000, 0.79607844, 0.25098041],
                [0.63921571, 1.00000000, 0.25098041],
                [0.25098041, 1.00000000, 0.42745098],
                [0.25098041, 1.00000000, 0.99215686],
                [0.25098041, 0.44313726, 1.00000000],
                [0.61960787, 0.25098041, 1.00000000],
                [1.00000000, 0.25098041, 0.81568629],
            ],
            [
                [1.00000000, 0.12549020, 0.12549020],
                [1.00000000, 0.76470590, 0.12549020],
                [0.57647061, 1.00000000, 0.12549020],
                [0.12549020, 1.00000000, 0.32941177],
                [0.12549020, 1.00000000, 0.98823529],
                [0.12549020, 0.35294119, 1.00000000],
                [0.55686277, 0.12549020, 1.00000000],
                [1.00000000, 0.12549020, 0.78431374],
            ],
            [
                [1.00000000, 0.00000000, 0.00000000],
                [1.00000000, 0.72941178, 0.00000000],
                [0.51764709, 1.00000000, 0.00000000],
                [0.00000000, 1.00000000, 0.23529412],
                [0.00000000, 1.00000000, 0.98823529],
                [0.00000000, 0.25882354, 1.00000000],
                [0.49411765, 0.00000000, 1.00000000],
                [1.00000000, 0.00000000, 0.75294119],
            ],
        ]);
        let output = image
            .decode_rgb_tensor::<NdArray>(&Default::default())
            .unwrap();
        output.into_data().assert_eq(&target.into_data(), true);
    }

    #[test]
    fn encode_rgb_tensor_on_mismatched_tensor_shape() {
        use super::*;
        use burn_ndarray::NdArray;

        let source = Tensor::<NdArray, 3>::ones([8, 6, 4], &Default::default());
        let mut image = Image::default();

        let target = (vec![8, 6, 4], vec![8, 6, 3]);
        let output = matches!(
            image.encode_rgb_tensor(source).unwrap_err(),
            Error::MismatchedTensorShape(output_0, output_1)
            if output_0 == target.0 && output_1 == target.1,
        );
        let target = true;
        assert_eq!(output, target);
    }
}
