pub use super::*;

use crate::function::read_bytes;
use std::{io::Read, iter};

impl DecoderWith<&Header> for Payload {
    type Err = Error;

    fn decode_with(
        reader: &mut impl Read,
        init: &Header,
    ) -> Result<Self, Self::Err> {
        assert!(
            !init.format.is_ascii(),
            "Unimplemented: ASCII format decoding"
        );

        let data = init
            .elements
            .values()
            .map(|elem| {
                let prop_count = elem.len();
                let prop_sizes = elem.property_sizes().collect::<Result<Vec<_>, _>>()?;
                let elem_size = prop_sizes.iter().sum::<usize>();

                // TODO: TODO: perf on universal size

                iter::repeat_n(elem_size, elem.count).try_fold(
                    vec![Vec::with_capacity(1 << 15); prop_count],
                    |mut props, elem_size| {
                        let mut data = read_bytes(reader, elem_size)?;
                        props.iter_mut().zip(prop_sizes.iter()).fold(
                            0,
                            |start, (prop, size)| {
                                let end = start + size;
                                // NOTE: The index is guaranteed to be valid
                                let datum = data.get_mut(start..end).unwrap();
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

        // NOTE: Currently, only scalar payload is implemented.
        let payload = ScalarPayload { data }.into();

        #[cfg(all(debug_assertions, not(test)))]
        log::debug!(target: "gausplat-loader::polygon::payload", "Payload::decode_with");

        Ok(payload)
    }
}
