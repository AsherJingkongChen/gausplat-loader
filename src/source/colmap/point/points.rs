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
        for (point_id, point) in self.iter().enumerate() {
            // Write point id.
            write_any(writer, &(point_id as u64))?;
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
    fn decode_zero_bytes() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[]);

        let points = Points::decode(&mut reader);
        assert!(points.is_err(), "{:#?}", points.unwrap());
    }

    #[test]
    fn decode_zero_entries() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let points = Points::decode(&mut reader).unwrap();
        assert!(points.is_empty());
    }

    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let mut reader = Cursor::new(&[
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x0f, 0xc5, 0x11, 0x70, 0x58,
            0xff, 0x3f, 0x32, 0x1e, 0xe3, 0x7d, 0x81, 0x6d, 0xdf, 0xbf, 0x55,
            0xff, 0x26, 0xb3, 0xd7, 0x21, 0xf5, 0x3f, 0x5d, 0x7b, 0x6f, 0xec,
            0x27, 0xb8, 0xb5, 0x63, 0xd1, 0xe8, 0x3f, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xc5, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00,
            0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0x0c,
            0xce, 0x7e, 0x3c, 0x4f, 0xf8, 0xbf, 0x8d, 0x83, 0xa1, 0xe9, 0x02,
            0xca, 0x95, 0xbf, 0x48, 0xad, 0xbc, 0x21, 0x59, 0x22, 0xf4, 0xbf,
            0x79, 0x8b, 0x7e, 0x8d, 0x5e, 0x60, 0xd0, 0x25, 0x2b, 0xd7, 0x3f,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0x17, 0x58, 0xaa, 0x26, 0xae,
            0xff, 0x3f, 0x27, 0x7b, 0x01, 0x1e, 0xbe, 0x17, 0xb7, 0x3f, 0xd9,
            0x13, 0x38, 0xf0, 0x87, 0xba, 0xf4, 0x3f, 0xc6, 0xba, 0x94, 0x29,
            0x19, 0x86, 0xe5, 0x22, 0x49, 0xf4, 0x3f, 0x03, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x5c, 0x00, 0x00, 0x00, 0xa4, 0x23, 0x00,
            0x00, 0x5a, 0x00, 0x00, 0x00, 0x92, 0x26, 0x00, 0x00, 0x5b, 0x00,
            0x00, 0x00, 0xb8, 0x26, 0x00, 0x00,
        ]);

        let points = Points::decode(&mut reader).unwrap();
        assert_eq!(points.len(), 3);
        assert_eq!(
            points[0],
            Point {
                position: [
                    1.9590912527209607,
                    -0.4910587052695262,
                    1.3207623479974775,
                ],
                color_rgb: [93, 123, 111],
            }
        );
        assert_eq!(
            points[1],
            Point {
                position: [
                    -1.5193448022189842,
                    -0.021278424749087633,
                    -1.25838578394435
                ],
                color_rgb: [121, 139, 126],
            }
        );
        assert_eq!(
            points[2],
            Point {
                position: [
                    1.9800173429552996,
                    0.09020603401721115,
                    1.295539797168422
                ],
                color_rgb: [198, 186, 148],
            }
        );
    }
}
