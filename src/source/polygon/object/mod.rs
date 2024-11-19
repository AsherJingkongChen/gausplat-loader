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
        use head::{FormatMetaVariant::*, PropertyMetaVariant::*};

        let head = Head::decode(reader)?;

        if head.format.variant.is_ascii() {
            unimplemented!("TODO: Decoding on ascii format");
        }

        let mut body = Body::default();

        head.iter_elements_and_properties().try_for_each(
            |((_, element), properties)| {
                let property_variants = properties
                    .map(|(id, property)| (id, &property.variant))
                    .collect::<Box<[_]>>();

                (0..element.size).try_for_each(|_| {
                    property_variants.iter().try_for_each(|(&id, property)| {
                        match property {
                            Scalar(scalar) => {
                                let step = scalar.size;
                                let value = read_bytes(reader, step)?;

                                let capacity = element.size * scalar.size;
                                body.get_scalar_mut(id, capacity)
                                    .expect("Unreachable")
                                    .extend(value);
                            },
                            List(list) => {
                                let step = list.count.size;
                                let count: usize = match head.format.variant {
                                    BinaryLittleEndian => {
                                        reader.read_uint::<LE>(step)
                                    },
                                    Ascii => unreachable!(),
                                    BinaryBigEndian => {
                                        reader.read_uint::<BE>(step)
                                    },
                                }?
                                .try_into()?;
                                let step = count * list.value.size;
                                let value = read_bytes(reader, step)?;

                                let capacity = element.size * list.value.size;
                                body.get_list_mut(id, capacity)
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
        use head::{FormatMetaVariant::*, PropertyMetaVariant::*};

        if self.head.format.variant.is_ascii() {
            unimplemented!("TODO: Encoding on ascii format");
        }

        self.head.encode(writer)?;

        self.head.iter_elements_and_properties().try_for_each(
            |((_, element), properties)| {
                let property_variants = properties
                    .map(|(id, property)| (id, &property.variant))
                    .collect::<Box<[_]>>();

                (0..element.size).try_for_each(|element_index| {
                    property_variants.iter().try_for_each(|(&id, property)| {
                        match property {
                            Scalar(scalar) => {
                                let step = scalar.size;
                                let offset = element_index * step;
                                let value = &self
                                    .body
                                    .get_scalar(id)
                                    .expect("Unreachable")
                                    [offset..offset + step];

                                writer.write_all(value)?;
                            },
                            List(list) => {
                                let value = self
                                    .body
                                    .get_list(id)
                                    .expect("Unreachable")
                                    .get(element_index)
                                    .expect("Unreachable");
                                let count = (value.len() / list.value.size) as u64;
                                let step = list.count.size;

                                match self.head.format.variant {
                                    BinaryLittleEndian => {
                                        writer.write_uint::<LE>(count, step)?;
                                    },
                                    Ascii => unreachable!(),
                                    BinaryBigEndian => {
                                        writer.write_uint::<BE>(count, step)?;
                                    },
                                }

                                writer.write_all(value)?;
                            },
                        }

                        Ok::<(), Self::Err>(())
                    })
                })?;

                Ok::<(), Self::Err>(())
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_and_encode_on_binary_be() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
        );
        let reader = &mut Cursor::new(source);
        let output = Object::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        output.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        output.iter().zip(target.iter()).enumerate().for_each(
            |(index, (output, target))| {
                assert_eq!(output, target, "index: {}", index);
            },
        );
    }

    #[test]
    fn decode_and_encode_on_binary_le() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let output = Object::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        output.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        output.iter().zip(target.iter()).enumerate().for_each(
            |(index, (output, target))| {
                assert_eq!(output, target, "index: {}", index);
            },
        );
    }

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
