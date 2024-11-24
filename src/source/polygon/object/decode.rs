pub use super::*;

use crate::function::read_bytes;
use std::{io::Read, iter};

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let header = Header::decode(reader)?;
        assert!(!header.format.is_ascii(), "Unimplemented: ASCII format");

        let data = header
            .values()
            .map(|element| {
                let prop_count = element.len();
                let prop_sizes = element
                    .values()
                    .map(|prop| {
                        prop.try_unwrap_scalar_ref()
                            .map_err(|err| {
                                // FAIL: List prop
                                InvalidKind(err.input.to_string())
                            })?
                            .size()
                            .ok_or_else(|| InvalidKind(prop.kind.to_string()))
                        // FAIL: Unknown scalar
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let elem_count = element.count;
                let elem_size = prop_sizes.iter().sum::<usize>();

                iter::repeat_n(elem_size, elem_count).try_fold(
                    {
                        let mut props = Vec::with_capacity(prop_count);
                        props.resize(prop_count, Vec::<u8>::with_capacity(1 << 14));
                        props
                    },
                    |mut props, elem_size| {
                        // FAIL: No byte
                        let mut elem = read_bytes(reader, elem_size)?;
                        props.iter_mut().zip(prop_sizes.iter()).fold(
                            Default::default(),
                            |start, (prop, size)| {
                                let end = start + size;
                                let bytes = elem.get_mut(start..end).unwrap();
                                // JUMP: Different endian
                                if !header.format.is_binary_native_endian() {
                                    bytes.reverse();
                                }
                                prop.extend_from_slice(bytes);
                                end
                            },
                        );
                        Ok(props)
                    },
                )
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let payload = ScalarPayload { data }.into();

        Ok(Self { header, payload })
    }
}

#[cfg(test)]
mod tests {
    #[test]

    fn decode_on_example_triangle() {

    }
}
