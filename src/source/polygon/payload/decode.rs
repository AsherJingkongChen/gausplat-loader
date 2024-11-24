pub use super::*;

use crate::function::read_bytes;
use std::{io::Read, iter};

impl DecoderWith<&Header> for Payload {
    type Err = Error;

    fn decode_with(
        reader: &mut impl Read,
        init: &Header,
    ) -> Result<Self, Self::Err> {
        debug_assert!(
            !init.format.is_ascii(),
            "Unimplemented: ASCII format decoding"
        );

        let data = init
            .values()
            .map(|element| {
                let prop_count = element.len();
                let prop_sizes = element
                    .values()
                    .map(|prop| {
                        debug_assert!(
                            prop.is_scalar(),
                            "Unimplemented: Non-scalar property decoding"
                        );
                        prop.try_unwrap_scalar_ref()
                            .unwrap()
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
                        props.resize(prop_count, Vec::<u8>::with_capacity(1 << 15));
                        props
                    },
                    |mut props, elem_size| {
                        // FAIL: No byte
                        let mut elem = read_bytes(reader, elem_size)?;
                        props.iter_mut().zip(prop_sizes.iter()).fold(
                            0,
                            |start, (prop, size)| {
                                let end = start + size;
                                // NOTE: The index is guaranteed to be valid
                                let bytes = elem.get_mut(start..end).unwrap();
                                // JUMP: Different endian
                                // TODO: perf
                                if !init.format.is_binary_native_endian() {
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

        Ok(payload)
    }
}
