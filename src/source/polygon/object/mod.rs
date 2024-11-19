pub mod id;

pub use super::{body, head, Body, Head};
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use id::*;
pub use indexmap::IndexMap;

use crate::function::read_bytes;
use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};
use std::io::{Read, Write};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Object {
    pub head: Head,
    pub body: Body,
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use head::{FormatMetaVariant::*, PropertyMetaVariant};

        // Decoding the head

        let head = Head::decode(reader)?;

        if head.format.variant.is_ascii() {
            unimplemented!("TODO: Decoding on ascii format");
        }

        // Decoding the body

        let mut body = Body::default();

        head.iter_elements_and_properties().try_for_each(
            |((_, element), properties)| {
                let property_variants = properties
                    .map(|(id, property)| (id, &property.variant))
                    .collect::<Box<[_]>>();

                (0..element.size).try_for_each(|_| {
                    property_variants.iter().try_for_each(|(&id, property)| {
                        match property {
                            PropertyMetaVariant::Scalar(scalar) => {
                                let property_size = scalar.size;
                                let data_size = element.size * property_size;
                                let value = read_bytes(reader, property_size)?;

                                body.get_scalar_mut(id, data_size)
                                    .expect("Unreachable")
                                    .extend(value);
                            },
                            PropertyMetaVariant::List(list) => {
                                let count_size = list.count.size;
                                let value_count: usize =
                                    match head.format.variant {
                                        BinaryLittleEndian => {
                                            reader.read_uint::<LE>(count_size)
                                        },
                                        Ascii => unreachable!(),
                                        BinaryBigEndian => {
                                            reader.read_uint::<BE>(count_size)
                                        },
                                    }?
                                    .try_into()?;
                                let value_size = list.value.size;
                                let property_size = value_size * value_count;
                                let data_size_estimated = element.size
                                    * property_size.max(value_size);
                                let value = read_bytes(reader, property_size)?;

                                body.get_list_mut(id, data_size_estimated)
                                    .expect("Unreachable")
                                    .push(value.into());
                            },
                        };

                        Ok::<(), Self::Err>(())
                    })
                })?;

                Ok::<(), Self::Err>(())
            },
        )?;

        Ok(Self { head, body })
    }
}

impl Encoder for Object {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        self.head.encode(writer)?;

        // self.body.data_map.values().try_for_each(|data| {
        //     match &data.variant {
        //         DataVariant::Scalar(scalar) => {
        //             scalar.iter().try_for_each(|value| {
        //                 // writer.write_all(value)?;
        //                 // Ok(())
        //             })
        //         },
        //         DataVariant::List(list) => {
        //             // list.iter().try_for_each(|value| {
        //             //     writer.write_all(value)?;
        //             //     Ok(())
        //             // })
        //         },
        //     }
        // })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_on_binary_big_endian() {}
    #[test]
    fn decode_on_binary_little_endian() {}
    #[test]
    fn decode_on_empty_element() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/empty-element.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let object = Object::decode(reader).unwrap();

        let target = 2;
        let output = object.head.iter_elements().count();
        assert_eq!(output, target);

        let target = 3;
        let output = object
            .head
            .meta_map
            .values()
            .filter(|meta| meta.variant.is_element())
            .count();
        assert_eq!(output, target);

        let target = 11;
        let output = object
            .head
            .meta_map
            .values()
            .filter(|meta| meta.variant.is_property())
            .count();
        assert_eq!(output, target);

        let target = 5;
        let output = object.body.data_map.len();
        assert_eq!(output, target);
    }
}
