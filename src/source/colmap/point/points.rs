pub use super::*;

pub type Points = Vec<Point>;

impl Decoder for Points {
    type Err = Error;

    #[inline]
    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let reader = &mut BufReader::new(reader);

        let point_count = reader.read_u64::<LE>()?;
        let points = (0..point_count)
            .map(|_| {
                // Skip point id
                advance(reader, 8)?;
                Point::decode(reader)
            })
            .collect();

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::colmap::point", "Points::decode");

        points
    }
}

impl Encoder for Points {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        let writer = &mut BufWriter::new(writer);

        writer.write_u64::<LE>(self.len() as u64)?;
        self.iter()
            .enumerate()
            .try_for_each(|(point_index, point)| {
                // Write point index to point id
                writer.write_u64::<LE>(point_index as u64)?;
                point.encode(writer)
            })?;

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::colmap::point", "Points::encode");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;

        let source = &include_bytes!("../../../../examples/data/colmap/0/points3D.bin")[..];
        let mut reader = std::io::Cursor::new(source);

        let targets = vec![
            Point {
                position: [1.5724178716968433, 1.3564240230221387, 1.7538898806469643],
                color_rgb: [120, 119, 93],
            },
            Point {
                position: [2.0051609433534487, -7.565764755951541, 13.772794612423496],
                color_rgb: [56, 63, 27],
            },
            Point {
                position: [0.903303165177694, 1.3450047957050342, 0.7132061653263634],
                color_rgb: [120, 128, 81],
            },
            Point {
                position: [-0.2538381433102123, 2.0167046225357734, 0.20054194403740716],
                color_rgb: [89, 89, 82],
            },
            Point {
                position: [1.9721978366381463, 2.0600807450742864, 0.9798867439849316],
                color_rgb: [64, 62, 56],
            },
            Point {
                position: [-0.7142955735092194, 0.3243117479195404, 1.4634002798548704],
                color_rgb: [55, 59, 26],
            },
            Point {
                position: [-5.274968015907949, 0.8415210125122136, -0.6782984199914783],
                color_rgb: [59, 57, 40],
            },
            Point {
                position: [5.82378588291083, 0.5095604394582246, 4.241006457627927],
                color_rgb: [71, 73, 43],
            },
            Point {
                position: [6.394665813759088, 1.1211348999682709, 3.4997745196316528],
                color_rgb: [143, 144, 117],
            },
            Point {
                position: [1.5805116148624903, 5.40206892716795, -19.4210684817658],
                color_rgb: [58, 67, 50],
            },
        ];
        let output = Points::decode(&mut reader).unwrap();
        assert_eq!(output, targets);
    }

    #[test]
    fn decode_on_zero_bytes() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[]);

        Points::decode(&mut reader).unwrap_err();
    }

    #[test]
    fn decode_on_zero_entry() {
        use super::*;

        let mut reader = std::io::Cursor::new(&[0, 0, 0, 0, 0, 0, 0, 0]);

        let target = true;
        let output = Points::decode(&mut reader).unwrap().is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn encode() {
        use super::*;

        let source = [
            Point {
                position: [-9.653762040829593, -4.102401892127109, 9.599685045896118],
                color_rgb: [47, 51, 30],
            },
            Point {
                position: [5.487944921401847, 0.2107494446297745, 3.114260873278527],
                color_rgb: [165, 169, 126],
            },
            Point {
                position: [0.1410007471542446, 0.291254708094473, 2.2554270470753965],
                color_rgb: [121, 125, 94],
            },
            Point {
                position: [-0.970841016641282, -0.48531157645971296, 2.3516242254018627],
                color_rgb: [96, 96, 91],
            },
            Point {
                position: [-0.8143227596488996, 3.1710185435453306, 0.3694397529877653],
                color_rgb: [141, 139, 136],
            },
            Point {
                position: [1.157534330380484, 1.508798212187828, 0.9037922130535186],
                color_rgb: [131, 136, 98],
            },
            Point {
                position: [5.834357348282835, 1.4493333604378096, 3.1080390945391643],
                color_rgb: [151, 151, 147],
            },
            Point {
                position: [-0.24065866398375135, 0.1763233421385975, 1.6066914460314323],
                color_rgb: [141, 147, 89],
            },
            Point {
                position: [0.7556535574483431, 0.6682392592540607, 3.120770469139577],
                color_rgb: [153, 149, 137],
            },
            Point {
                position: [
                    -1.9299760562484711,
                    -0.37688731833688194,
                    0.8368212073339936,
                ],
                color_rgb: [84, 88, 78],
            },
        ]
        .into_iter()
        .collect::<Points>();

        let target = &include_bytes!("../../../../examples/data/colmap/1/points3D.bin")[..];
        let mut writer = std::io::Cursor::new(vec![]);
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_zero_entry() {
        use super::*;

        let source = Points::default();

        let target = &[0, 0, 0, 0, 0, 0, 0, 0];
        let mut writer = std::io::Cursor::new(vec![]);
        source.encode(&mut writer).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }
}
