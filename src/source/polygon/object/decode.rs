pub use super::*;

use std::io::{BufReader, Read};

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let reader = &mut BufReader::new(reader);
        let header = Header::decode(reader)?;
        let payload = Payload::decode_with(reader, &header)?;
        Ok(Self { header, payload })
    }
}

#[cfg(test)]
mod tests {
    // use std::io::Cursor;

    #[test]
    fn decode_on_example_triangle() {}

    // TODO:
    // - encode object tests
    // - decode object tests
    // - decode object and access properties tests
}
