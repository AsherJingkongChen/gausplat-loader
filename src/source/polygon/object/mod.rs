pub mod id;

pub use super::{body, head, Body, Head};
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use id::*;
pub use indexmap::IndexMap;

use crate::function::read_bytes;
use std::io::Read;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Object {
    pub head: Head,
    pub body: Body,
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use body::{Data, DataVariant, ListData, ScalarData};
        use head::PropertyMetaVariant;

        // Decoding the head

        let head = Head::decode(reader)?;

        // TODO: This requires byte-order-aware decoding utilities.
        if !head.format.variant.is_binary_little_endian() {
            unimplemented!(
                "TODO: Decoding on formats other than binary little-endian"
            );
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
                                PropertyMetaVariant::List(list) => {
                                    let count_size = list.count.size;
                                    let value_size = list.value.size;

                                    // NOTE: It assumes that the incoming bytes are in LE order.
                                    // TODO: Bad smell. It should be byte-order-aware.
                                    let mut value_count =
                                        [0; size_of::<usize>()];
                                    value_count.copy_from_slice(&read_bytes(
                                        reader, count_size,
                                    )?);
                                    let value_count =
                                        usize::from_le_bytes(value_count);

                                    let data_size_estimated = element_size
                                        * value_size
                                        * value_count.max(1);
                                    let property_size =
                                        value_size * value_count;

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
                                        .push(
                                            read_bytes(reader, property_size)?
                                                .into(),
                                        );
                                },
                                PropertyMetaVariant::Scalar(scalar) => {
                                    let property_size = scalar.size;
                                    let data_size =
                                        element_size * property_size;

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
                                        .extend(read_bytes(
                                            reader,
                                            property_size,
                                        )?);
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
