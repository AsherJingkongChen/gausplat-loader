pub use super::*;

use std::io::Read;

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let header = Header::decode(reader)?;
        let payload = Payload::decode_with(reader, &header)?;
        Ok(Self { header, payload })
    }
}

#[cfg(test)]
mod tests {
    // use std::io::Cursor;

    #[test]
    fn decode_on_example_triangle() {
    }

    // TODO:
    // - encode implementations
    // - decode object tests
    // - decode object and access properties tests
}
