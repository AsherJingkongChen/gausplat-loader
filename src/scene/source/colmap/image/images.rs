pub use super::Image;
pub use crate::function::Decoder;
use crate::{error::Error, function::read_slice};
use std::io::{BufReader, Read};

pub type Images = std::collections::HashMap<u32, Image>;

impl Decoder for Images {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let reader = &mut BufReader::new(reader);
        let image_count = read_slice::<u64, 1>(reader)?[0] as usize;

        let images = (0..image_count)
            .map(|_| {
                let image = Image::decode(reader)?;
                Ok((image.image_id, image))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat_importer::scene", "colmap::Images::decode");

        images
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_zero_bytes() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[]);

        let images = Images::decode(&mut reader);
        assert!(images.is_err(), "{:#?}", images.unwrap());
    }

    #[test]
    fn decode_zero_entries() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let images = Images::decode(&mut reader);
        assert!(images.is_ok(), "{}", images.unwrap_err());

        let images = images.unwrap();
        assert!(images.is_empty());
    }

    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x55, 0x80, 0x33, 0x6f, 0x36, 0xfc, 0xee, 0x3f, 0x04, 0x1f,
            0x73, 0x55, 0x8a, 0x93, 0x96, 0xbf, 0x5a, 0x19, 0x42, 0xcb, 0xb9,
            0x9e, 0xcf, 0xbf, 0x4a, 0xcd, 0xe1, 0x81, 0xd2, 0xdc, 0x9e, 0x3f,
            0xa6, 0xdb, 0x19, 0x7c, 0x4b, 0x95, 0xea, 0x3f, 0x5c, 0x45, 0xe3,
            0x6d, 0x6a, 0x17, 0xdb, 0x3f, 0x6b, 0x8b, 0xfe, 0x3c, 0x7b, 0xe1,
            0x12, 0x40, 0x01, 0x00, 0x00, 0x00, 0x30, 0x30, 0x30, 0x30, 0x31,
            0x2e, 0x6a, 0x70, 0x67, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xc0, 0xb5, 0x7e, 0xd4, 0x80, 0xe5, 0x7c, 0x40, 0xa0,
            0x10, 0x2f, 0xd7, 0xab, 0xc6, 0x5c, 0x40, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0x02, 0x00, 0x00, 0x00, 0x15, 0x34, 0x57,
            0x48, 0x24, 0x28, 0xef, 0x3f, 0x20, 0x30, 0xc2, 0x81, 0x61, 0xee,
            0x96, 0xbf, 0xba, 0xe0, 0x12, 0x04, 0x5c, 0xd2, 0xcc, 0xbf, 0x2a,
            0x9f, 0x43, 0x4b, 0x99, 0x02, 0x9d, 0x3f, 0x8e, 0xc7, 0xfc, 0xff,
            0x7d, 0x9b, 0xf0, 0x3f, 0xd4, 0x92, 0x74, 0x4f, 0x24, 0xe4, 0xda,
            0x3f, 0x47, 0xda, 0x15, 0xb9, 0x51, 0xb6, 0x12, 0x40, 0x02, 0x00,
            0x00, 0x00, 0x30, 0x30, 0x30, 0x30, 0x32, 0x2e, 0x6a, 0x70, 0x67,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);

        let images = Images::decode(&mut reader);
        assert!(images.is_ok(), "{}", images.unwrap_err());

        let images = images.unwrap();
        assert_eq!(images.len(), 2);
        assert_eq!(
            images.get(&1),
            Some(&Image {
                image_id: 1,
                rotation: [
                    0.9682876750848758,
                    -0.022047196832118324,
                    -0.24703142571179165,
                    0.030139245202417876
                ],
                translation: [
                    0.8307244705055055,
                    0.42330418330410224,
                    4.720196679149789
                ],
                camera_id: 1,
                file_name: "00001.jpg".into(),
            })
        );
        assert_eq!(
            images.get(&2),
            Some(&Image {
                image_id: 2,
                rotation: [
                    0.9736501133826346,
                    -0.022393725914796048,
                    -0.2251696605578724,
                    0.02833022615314388
                ],
                translation: [
                    1.037961959792003,
                    0.42017467269242315,
                    4.678046123465328
                ],
                camera_id: 2,
                file_name: "00002.jpg".into(),
            })
        );
    }
}
