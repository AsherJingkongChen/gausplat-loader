pub use super::Image;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::{read_any, write_any};
use std::io::{BufReader, BufWriter, Read, Write};

pub type Images = crate::collection::IndexMap<u32, Image>;

impl Decoder for Images {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let reader = &mut BufReader::new(reader);

        let image_count = read_any::<u64>(reader)? as usize;
        let images = (0..image_count)
            .map(|_| {
                let image = Image::decode(reader)?;
                Ok((image.image_id, image))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::image", "Images::decode");

        images
    }
}

impl Encoder for Images {
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error> {
        let writer = &mut BufWriter::new(writer);

        write_any(writer, &(self.len() as u64))?;
        self.values().try_for_each(|image| image.encode(writer))?;

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::image", "Images::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;

        let source =
            include_bytes!("../../../../examples/data/colmap/0/images.bin");
        let mut reader = std::io::Cursor::new(source);

        let targets = [
            (
                1,
                Image {
                    image_id: 1,
                    quaternion: [
                        0.988495209314160400,
                        0.050344076688825165,
                        0.125084821482361330,
                        0.068530887489000340,
                    ],
                    translation: [
                        -0.1032706556958793,
                        -1.8918304315849880,
                        3.02849770919350700,
                    ],
                    camera_id: 1,
                    file_name: "001.png".into(),
                },
            ),
            (
                2,
                Image {
                    image_id: 2,
                    quaternion: [
                        0.989048876393201000,
                        0.050933350734933304,
                        0.122353383332439720,
                        0.064944310569625920,
                    ],
                    translation: [
                        -0.09420952183337841,
                        -1.87884325760431200,
                        2.996173726996660000,
                    ],
                    camera_id: 1,
                    file_name: "002.png".into(),
                },
            ),
            (
                3,
                Image {
                    image_id: 3,
                    quaternion: [
                        0.989741809691070100,
                        0.051584383034821296,
                        0.118291825488558590,
                        0.061296375088148790,
                    ],
                    translation: [
                        -0.09340967030420796,
                        -1.86571424964666720,
                        2.962536046879310000,
                    ],
                    camera_id: 1,
                    file_name: "003.png".into(),
                },
            ),
            (
                4,
                Image {
                    image_id: 4,
                    quaternion: [
                        0.99044336453881290,
                        0.05219662279078869,
                        0.11381991984156653,
                        0.05781418560813428,
                    ],
                    translation: [
                        -0.09346365413936121,
                        -1.85262917468621540,
                        2.929815138064468000,
                    ],
                    camera_id: 1,
                    file_name: "004.png".into(),
                },
            ),
            (
                5,
                Image {
                    image_id: 5,
                    quaternion: [
                        0.99120592907542140,
                        0.05271161079731468,
                        0.10853412337216885,
                        0.05433816629881809,
                    ],
                    translation: [
                        -0.09795997578278112,
                        -1.83924911751858080,
                        2.898707117916470700,
                    ],
                    camera_id: 1,
                    file_name: "005.png".into(),
                },
            ),
        ]
        .into_iter()
        .collect::<Images>();
        let output = Images::decode(&mut reader).unwrap();
        assert_eq!(output, targets);
    }

    #[test]
    fn decode_on_zero_bytes() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[]);

        Images::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn decode_on_zero_entry() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let target = true;
        let output = Images::decode(&mut reader).unwrap().is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn encode() {
        use super::*;

        let source = [
            (
                1,
                Image {
                    image_id: 1,
                    quaternion: [
                        0.9927344237585644,
                        0.05356216586335562,
                        0.09685107539050428,
                        0.04721575445209409,
                    ],
                    translation: [
                        -0.11322076094648384,
                        -1.810127034526175,
                        2.841519901424116,
                    ],
                    camera_id: 1,
                    file_name: "00001.png".into(),
                },
            ),
            (
                2,
                Image {
                    image_id: 2,
                    quaternion: [
                        0.9941554206814687,
                        0.054534905964818324,
                        0.08426611828512223,
                        0.03975128765484755,
                    ],
                    translation: [
                        -0.12798266308207282,
                        -1.7803443039067588,
                        2.7869433888185773,
                    ],
                    camera_id: 1,
                    file_name: "00002.png".into(),
                },
            ),
            (
                3,
                Image {
                    image_id: 3,
                    quaternion: [
                        0.9958116997894283,
                        0.055285286303065485,
                        0.0652576279839286,
                        0.03231157177837366,
                    ],
                    translation: [
                        -0.17219107261746,
                        -1.7518295830638597,
                        2.733216794563995,
                    ],
                    camera_id: 1,
                    file_name: "00003.png".into(),
                },
            ),
            (
                4,
                Image {
                    image_id: 4,
                    quaternion: [
                        0.9968807229913766,
                        0.05572515448979502,
                        0.0502476290699628,
                        0.024468491128920144,
                    ],
                    translation: [
                        -0.1816701609715575,
                        -1.7203007075836567,
                        2.6867445977350486,
                    ],
                    camera_id: 1,
                    file_name: "00004.png".into(),
                },
            ),
            (
                5,
                Image {
                    image_id: 5,
                    quaternion: [
                        0.9976940774498098,
                        0.05641770582190205,
                        0.03422172317002589,
                        0.015888484969957376,
                    ],
                    translation: [
                        -0.18796144698191583,
                        -1.6942072228370366,
                        2.642389506923824,
                    ],
                    camera_id: 1,
                    file_name: "00005.png".into(),
                },
            ),
        ]
        .into_iter()
        .collect::<Images>();

        let target =
            include_bytes!("../../../../examples/data/colmap/1/images.bin");
        let mut writer = std::io::Cursor::new(Vec::new());
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_zero_entry() {
        use super::*;

        let source = Images::default();

        let target = &[0, 0, 0, 0, 0, 0, 0, 0];
        let mut writer = std::io::Cursor::new(Vec::new());
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }
}
