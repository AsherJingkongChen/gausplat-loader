pub mod id;

pub use super::{body, head, Body, Head};
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use id::*;
pub use indexmap::IndexMap;

use crate::function::read_bytes;
use byteorder::{ReadBytesExt, BE, LE};
use std::{io::Read, ops::Mul};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Object {
    pub head: Head,
    pub body: Body,
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use body::{Data, DataVariant, ListData, ScalarData};
        use head::{FormatMetaVariant::*, PropertyMetaVariant};

        // Decoding the head

        let head = Head::decode(reader)?;

        if head.format.variant.is_ascii() {
            unimplemented!("TODO: Decoding on ascii format");
        }

        // Decoding the body

        let mut body = Body::default();

        // Reading each element

        head.group.element_to_property_ids.iter().try_for_each(
            |(element_id, property_ids)| {
                // Reading the element size

                let element_size = head
                    .meta_map
                    .get(element_id)
                    .and_then(|meta| meta.variant.as_element())
                    .map(|element| element.size)
                    .expect("Unreachable");

                // Reading all properties metadata

                let properties = property_ids
                    .iter()
                    .filter_map(|property_id| {
                        head.meta_map
                            .get(property_id)
                            .and_then(|meta| meta.variant.as_property())
                            .map(|property| &property.variant)
                    })
                    .collect::<Vec<_>>();

                // Reading each property data

                (0..element_size).try_for_each(|_| {
                    property_ids.iter().zip(properties.iter()).try_for_each(
                        |(&property_id, property)| {
                            match property {
                                PropertyMetaVariant::Scalar(scalar) => {
                                    let property_size = scalar.size;
                                    let data_size =
                                        element_size.mul(property_size);

                                    let value =
                                        read_bytes(reader, property_size)?;

                                    body.data_map
                                        .entry(property_id)
                                        .or_insert_with(|| Data {
                                            id: property_id,
                                            variant: DataVariant::Scalar(
                                                ScalarData::with_capacity(
                                                    data_size,
                                                ),
                                            ),
                                        })
                                        .variant
                                        .as_scalar_mut()
                                        .expect("Unreachable")
                                        .extend(value);
                                },
                                PropertyMetaVariant::List(list) => {
                                    let count_size = list.count.size;
                                    let value_size = list.value.size;
                                    let value_count: usize =
                                        match head.format.variant {
                                            BinaryLittleEndian => reader
                                                .read_uint::<LE>(count_size),
                                            Ascii => unreachable!(),
                                            BinaryBigEndian => reader
                                                .read_uint::<BE>(count_size),
                                        }?
                                        .try_into()?;
                                    let property_size =
                                        value_size.mul(value_count);
                                    let data_size_estimated = element_size
                                        .mul(property_size.max(value_size));

                                    let value =
                                        read_bytes(reader, property_size)?;

                                    body.data_map
                                        .entry(property_id)
                                        .or_insert_with(|| Data {
                                            id: property_id,
                                            variant: DataVariant::List(
                                                ListData::with_capacity(
                                                    data_size_estimated,
                                                ),
                                            ),
                                        })
                                        .variant
                                        .as_list_mut()
                                        .expect("Unreachable")
                                        .push(value.into());
                                },
                            };

                            Ok::<(), Self::Err>(())
                        },
                    )
                })?;

                Ok::<(), Self::Err>(())
            },
        )?;

        // head -> element collection -> property collection

        Ok(Self { head, body })
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
        let output = Object::decode(reader).unwrap();
        println!("{:#?}", output);
    }
}
