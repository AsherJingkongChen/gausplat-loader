pub mod id;

pub use super::{body, head, Body, Head};
use crate::function::read_bytes;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use id::*;
pub use indexmap::IndexMap;

use std::io::Read;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Object {
    pub head: Head,
    pub body: Body,
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        use body::{BodyBlock, BodyBlockVariant, ScalarBodyBlock};
        use head::PropertyBlockVariant;

        // Decoding the head

        let head = Head::decode(reader)?;

        // Decoding the body

        let mut body = Body::default();

        // Reading each element

        head.group.element_to_property_ids.iter().try_for_each(
            |(element_id, property_ids)| {
                // Reading the element size

                let element_size = head
                    .blocks
                    .get(element_id)
                    .and_then(|block| block.variant.as_element())
                    .map(|element| element.size)
                    .expect("Unreachable");

                // Reading all properties metadata

                let properties = property_ids
                    .iter()
                    .filter_map(|property_id| {
                        head.blocks
                            .get(property_id)
                            .and_then(|block| block.variant.as_property())
                            .map(|property| &property.variant)
                    })
                    .collect::<Vec<_>>();

                // Reading each property data

                (0..element_size).try_for_each(|_| {
                    property_ids.iter().zip(properties.iter()).try_for_each(
                        |(&property_id, property)| {
                            match property {
                                PropertyBlockVariant::List(list) => {
                                    let count_size = list.count.size;
                                    let value_size = list.value.size;
                                },
                                PropertyBlockVariant::Scalar(scalar) => {
                                    let property_size = scalar.size;
                                    let block_size =
                                        element_size * property_size;

                                    body.blocks
                                        .entry(property_id)
                                        .or_insert_with(|| BodyBlock {
                                            id: property_id,
                                            variant: BodyBlockVariant::Scalar(
                                                ScalarBodyBlock::with_capacity(
                                                    block_size,
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
