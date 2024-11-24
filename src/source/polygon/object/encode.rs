pub use super::*;

use std::{
    io::{BufWriter, Write},
    iter,
};

impl Encoder for Object {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        let writer = &mut BufWriter::new(writer);

        // FAIL: No ascii string in header
        self.header.encode(writer)?;
        debug_assert!(
            !self.header.format.is_ascii(),
            "Unimplemented: ASCII format encoding"
        );

        let elements = self.get_elements();
        elements
            .0
            .values()
            .zip(elements.1.iter())
            .try_for_each(|(elem, elem_data)| {
                // LOOP: each element
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
                // FAIL: Same

                iter::repeat_n((), elem.count)
                    .try_fold(vec![0; prop_count], |mut prop_offsets, _| {
                        // LOOP: each element count
                        prop_offsets
                            .iter_mut()
                            .zip(prop_sizes.iter().zip(elem_data.iter()))
                            .try_for_each(|(offset, (size, data))| {
                                // LOOP: each property
                                let start = *offset;
                                let end = start + size;
                                *offset = end;

                                // FAIL: Smaller element count
                                let datum = data
                                    .get(start..end)
                                    .ok_or_else(|| OutOfBounds(end, elem.count * size))?;
                                // JUMP: Different endian
                                if self.header.format.is_binary_native_endian() {
                                    writer.write_all(datum)
                                } else {
                                    let datum = &mut datum.to_owned();
                                    datum.reverse();
                                    writer.write_all(datum)
                                }?;
                                // FAIL: Write error

                                Ok::<_, Self::Err>(())
                            })?; // FAIL: Same
                        Ok(prop_offsets)
                    })
                    .map(drop)
            })
    }
}

#[cfg(test)]
mod tests {}
