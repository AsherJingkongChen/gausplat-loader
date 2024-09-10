pub use burn_tensor::TensorData;
pub use image::RgbImage;

pub trait IntoRgbImage {
    fn into_rgb_image(self) -> RgbImage;
}

pub trait IntoTensorData {
    fn into_tensor_data(self) -> TensorData;
}

impl IntoRgbImage for TensorData {
    /// ## Arguments
    ///
    /// * `self.shape` should be `[H, W, C]` where `C` is 3.
    ///
    /// ## Returns
    ///
    /// An RGB image with height `H` and width `W`.
    fn into_rgb_image(self) -> RgbImage {
        let dimension_count = self.shape.len();

        debug_assert_eq!(
            dimension_count, 3,
            "dimension_count should be 3, not {dimension_count}."
        );

        let height = self.shape[0] as u32;
        let width = self.shape[1] as u32;
        let channel_count = self.shape[2];

        debug_assert_eq!(
            channel_count, 3,
            "channel_count should be 3, not {channel_count}."
        );

        let value = self.convert::<u8>().bytes;

        RgbImage::from_raw(width, height, value)
            .expect("Unreachable on debug build")
    }
}

impl IntoTensorData for RgbImage {
    /// ## Returns
    ///
    /// A tensor data with shape of `[H, W, C]` where `C` is 3.
    fn into_tensor_data(self) -> TensorData {
        let height = self.height() as usize;
        let width = self.width() as usize;
        let channel_count = 3;
        let value = self.into_raw();

        TensorData::new(value, [height, width, channel_count])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn into_rgb_image() {
        use super::*;

        let tensor_data = TensorData::new(vec![0.0, 0.0, 0.0], [1, 1, 3]);
        let rgb_image = tensor_data.into_rgb_image();
        assert_eq!(rgb_image.get_pixel(0, 0).0, [0, 0, 0]);

        let tensor_data = TensorData::new(vec![255.0, 255.0, 255.0], [1, 1, 3]);
        let rgb_image = tensor_data.into_rgb_image();
        assert_eq!(rgb_image.get_pixel(0, 0).0, [255, 255, 255]);
    }

    #[test]
    fn into_tensor_data() {
        use super::*;

        let rgb_image = RgbImage::from_raw(1, 1, vec![0, 0, 0]).unwrap();
        let tensor_data = rgb_image.into_tensor_data();
        assert_eq!(tensor_data.shape, [1, 1, 3]);

        let rgb_image = RgbImage::from_raw(1, 1, vec![255, 255, 255]).unwrap();
        let tensor_data = rgb_image.into_tensor_data();
        assert_eq!(tensor_data.shape, [1, 1, 3]);
    }
}
