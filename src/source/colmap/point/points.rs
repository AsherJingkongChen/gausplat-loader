pub use super::Point;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::{advance, read_any, write_any};
use std::io::{BufReader, BufWriter, Read, Write};

pub type Points = Vec<Point>;

impl Decoder for Points {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let reader = &mut BufReader::new(reader);

        let point_count = read_any::<u64>(reader)? as usize;
        let points = (0..point_count)
            .map(|_| {
                // Read point id and ignore it.
                advance(reader, 8)?;
                Point::decode(reader)
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::point", "Points::decode");

        points
    }
}

impl Encoder for Points {
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error> {
        let writer = &mut BufWriter::new(writer);

        write_any(writer, &(self.len() as u64))?;
        for (point_index, point) in self.iter().enumerate() {
            // Write point id from index.
            write_any(writer, &(point_index as u64))?;
            point.encode(writer)?;
        }

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat::loader::colmap::point", "Points::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;

        let source = include_bytes!("../../../../examples/data/points3D.bin");
        let mut reader = std::io::Cursor::new(source);

        let target_count = 10;
        let targets = vec![
            Point {
                position: [
                    1.5724178716968433,
                    1.3564240230221387,
                    1.7538898806469643,
                ],
                color_rgb: [120, 119, 93],
            },
            Point {
                position: [
                    2.0051609433534487,
                    -7.565764755951541,
                    13.772794612423496,
                ],
                color_rgb: [56, 63, 27],
            },
            Point {
                position: [
                    0.903303165177694,
                    1.3450047957050342,
                    0.7132061653263634,
                ],
                color_rgb: [120, 128, 81],
            },
            Point {
                position: [
                    -0.2538381433102123,
                    2.0167046225357734,
                    0.20054194403740716,
                ],
                color_rgb: [89, 89, 82],
            },
            Point {
                position: [
                    1.9721978366381463,
                    2.0600807450742864,
                    0.9798867439849316,
                ],
                color_rgb: [64, 62, 56],
            },
            Point {
                position: [
                    -0.7142955735092194,
                    0.3243117479195404,
                    1.4634002798548704,
                ],
                color_rgb: [55, 59, 26],
            },
            Point {
                position: [
                    -5.274968015907949,
                    0.8415210125122136,
                    -0.6782984199914783,
                ],
                color_rgb: [59, 57, 40],
            },
            Point {
                position: [
                    5.82378588291083,
                    0.5095604394582246,
                    4.241006457627927,
                ],
                color_rgb: [71, 73, 43],
            },
            Point {
                position: [
                    6.394665813759088,
                    1.1211348999682709,
                    3.4997745196316528,
                ],
                color_rgb: [143, 144, 117],
            },
            Point {
                position: [
                    1.5805116148624903,
                    5.40206892716795,
                    -19.4210684817658,
                ],
                color_rgb: [58, 67, 50],
            },
        ];
        let outputs = Points::decode(&mut reader).unwrap();
        assert_eq!(outputs.len(), target_count);
        assert_eq!(outputs, targets);
    }

    #[test]
    fn decode_on_zero_bytes() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[]);

        Points::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn decode_on_zero_entries() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let outputs = Points::decode(&mut reader).unwrap();
        assert!(outputs.is_empty());
    }
}
