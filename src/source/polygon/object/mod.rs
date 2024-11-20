pub use super::*;
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::read_bytes;
use body::*;
use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};
use head::*;
use std::io::{Read, Write};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Object {
    pub head: Head,
    pub body: Body,
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let head = Head::decode(reader)?;
        assert!(!head.is_format_ascii(), "TODO: Decoding on ascii format");

        let mut body = Body::with_capacity(2);

        head.elements_and_properties().try_for_each(
            |(element, properties)| -> Result<(), Self::Err> {
                let properties = properties.collect::<Vec<_>>();

                let data = (0..element.size).try_fold(
                    properties
                        .iter()
                        .map(|property| {
                            match &***property {
                                PropertyMetaVariant::Scalar(scalar) => {
                                    DataVariant::Scalar(
                                        ScalarData::with_capacity(
                                            element.size * scalar.size,
                                        ),
                                    )
                                },
                                PropertyMetaVariant::List(list) => {
                                    DataVariant::List(ListData::with_capacity(
                                        element.size * list.value.size,
                                    ))
                                },
                            }
                            .into()
                        })
                        .collect(),
                    |mut data, _| -> Result<Vec<Data>, Self::Err> {
                        properties.iter().zip(data.iter_mut()).try_for_each(
                            |(property, datum)| -> Result<(), Self::Err> {
                                match &***property {
                                    PropertyMetaVariant::Scalar(scalar) => {
                                        let step = scalar.size;
                                        let value = read_bytes(reader, step)?;
                                        datum
                                            .as_scalar_mut()
                                            .expect("Unreachable")
                                            .extend(value);
                                    },
                                    PropertyMetaVariant::List(list) => {
                                        let step = list.count.size;
                                        let count: usize =
                                            match head.get_format() {
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
                                        datum
                                            .as_list_mut()
                                            .expect("Unreachable")
                                            .push(value.into());
                                    },
                                }
                                Ok(())
                            },
                        )?;
                        Ok(data)
                    },
                )?;
                Ok(body.push(data))
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
        assert!(
            !self.head.is_format_ascii(),
            "TODO: Encoding on ascii format"
        );

        self.head
            .elements_and_properties()
            .zip(self.body.iters())
            .try_for_each(
                |((element, properties), data)| -> Result<(), Self::Err> {
                    let properties = properties.collect::<Vec<_>>();
                    let data = data.collect::<Vec<_>>();

                    (0..element.size).try_for_each(
                        |element_index| -> Result<(), Self::Err> {
                            properties.iter().zip(data.iter()).try_for_each(
                                |(property, datum)| -> Result<(), Self::Err> {
                                    match &***property {
                                        PropertyMetaVariant::Scalar(scalar) => {
                                            let step = scalar.size;
                                            let offset = element_index * step;
                                            let value = datum
                                                .as_scalar()
                                                .expect("Unreachable")
                                                .get(offset..offset + step)
                                                .expect("TODO");
                                            writer.write_all(value)?;
                                        },
                                        PropertyMetaVariant::List(list) => {
                                            let value = datum
                                                .as_list()
                                                .expect("Unreachable")
                                                .get(element_index)
                                                .expect("TODO");
                                            let count = value
                                                .len()
                                                .div_euclid(list.value.size)
                                                as u64;
                                            let step = list.count.size;
                                            match self.head.get_format() {
                                                BinaryLittleEndian => {
                                                    writer.write_uint::<LE>(
                                                        count, step,
                                                    )?;
                                                },
                                                Ascii => unreachable!(),
                                                BinaryBigEndian => {
                                                    writer.write_uint::<BE>(
                                                        count, step,
                                                    )?;
                                                },
                                            }
                                            writer.write_all(value)?;
                                        },
                                    };
                                    Ok(())
                                },
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
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

        let target = 3;
        let output =
            object.head.iter().filter(|meta| meta.is_element()).count();
        assert_eq!(output, target);

        let target = 11;
        let output =
            object.head.iter().filter(|meta| meta.is_property()).count();
        assert_eq!(output, target);

        let target = 11;
        let output = object.body.iters().flatten().count();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_and_encode_on_empty_element() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/empty-element.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let object = Object::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        object.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        assert_eq!(output, target);
    }
}
