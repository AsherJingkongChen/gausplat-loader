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
            .elements
            .values()
            .map(|elem| {
                let prop_count = elem.len();
                let prop_sizes = elem
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
                let elem_size = prop_sizes.iter().sum::<usize>();

                iter::repeat_n(elem_size, elem.count).try_fold(
                    vec![Vec::with_capacity(1 << 15); prop_count],
                    |mut props, elem_size| {
                        // FAIL: No byte
                        let mut data = read_bytes(reader, elem_size)?;
                        props.iter_mut().zip(prop_sizes.iter()).fold(
                            0,
                            |start, (prop, size)| {
                                let end = start + size;
                                // NOTE: The index is guaranteed to be valid
                                let datum = data.get_mut(start..end).unwrap();
                                // JUMP: Different endian
                                if !init.format.is_binary_native_endian() {
                                    datum.reverse();
                                }
                                prop.extend_from_slice(datum);
                                end
                            },
                        );
                        Ok(props)
                    },
                )
            })
            .collect::<Result<_, Self::Err>>()?;

        let payload = ScalarPayload { data }.into();

        Ok(payload)
    }
}
