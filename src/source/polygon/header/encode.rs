pub use super::*;

use std::io::Write;

impl Encoder for Header {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        let header = self.to_string();
        if !header.is_ascii() {
            return Err(Error::InvalidAscii(header));
        }

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::polygon::header", "Header::encode");

        Ok(write!(writer, "{header}")?)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    /// This test **ensures** idempotence of header encoded outputs.
    #[test]
    fn encode_on_example_ok_and_idempotent() {
        use super::*;

        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.ascii.ply"
            )[..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
            )[..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
            )[..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/another-cube.greg-turk.zeros.binary-le.ply"
            )[..],
        );
        assert(
            &include_bytes!("../../../../examples/data/polygon/empty-element.ascii.ply")
                [..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/empty-element.binary-le.ply"
            )[..],
        );
        assert(
            &include_bytes!("../../../../examples/data/polygon/empty-head.ascii.ply")[..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/supported-data-types-common.ply"
            )[..],
        );
        assert(
            &include_bytes!(
                "../../../../examples/data/polygon/supported-data-types-special.ply"
            )[..],
        );
        assert(
            &include_bytes!("../../../../examples/data/polygon/triangle.binary-le.ply")[..],
        );
        assert(
            &include_bytes!("../../../../examples/data/polygon/valid-keyword.ascii.ply")
                [..],
        );

        fn assert(source: &[u8]) {
            let reader = &mut Cursor::new(source);

            let header = Header::decode(reader).unwrap();
            let writer = &mut vec![];
            header.encode(writer).unwrap();
            let output_1 = writer.to_owned();

            let header = Header::decode(&mut Cursor::new(writer)).unwrap();
            let writer = &mut vec![];
            header.encode(writer).unwrap();
            let output_2 = writer.to_owned();

            assert_eq!(output_1, output_2);

            let target = true;
            let output = output_1.is_ascii();
            assert_eq!(output, target);

            let target = true;
            let output = output_1.len() > 4 && output_1.len() <= source.len();
            assert_eq!(output, target);
        }
    }

    #[test]
    fn encode_on_empty_slice() {
        use super::*;

        Header::default().encode(&mut &mut [][..]).unwrap_err();
    }

    #[test]
    fn encode_on_invalid_ascii() {
        use super::*;

        let mut header = Header::default();
        header.version = "\u{2077}".into();

        header.encode(&mut &mut [][..]).unwrap_err();
    }
}
