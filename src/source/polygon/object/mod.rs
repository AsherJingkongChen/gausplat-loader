pub use super::{
    body::{ElementData, PropertyData},
    head::{CommentMeta, ElementMeta, FormatMetaVariant, ObjInfoMeta, PropertyMeta},
    *,
};
pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::read_bytes;
use ascii::AsAsciiStr;
use body::{Data, DataVariant, ListData, ScalarData};
use byteorder::{ReadBytesExt, WriteBytesExt, BE, LE};
use head::{FormatMetaVariant::*, PropertyMetaVariant};
use std::{
    io::{Read, Write},
    ops,
};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Object {
    head: Head,
    body: Body,
}

impl Object {
    #[inline]
    pub const fn new(
        head: Head,
        body: Body,
    ) -> Self {
        Self { head, body }
    }
}

// Filtered accessors
impl Object {
    #[inline]
    pub fn get_comment<K: AsRef<[u8]>>(
        &self,
        part: K,
    ) -> Option<&CommentMeta> {
        let part = std::str::from_utf8(part.as_ref()).ok()?;
        self.iter_comment()
            .find(|comment| comment.as_str().contains(part))
    }

    #[inline]
    pub fn get_comment_mut<K: AsRef<[u8]>>(
        &mut self,
        part: K,
    ) -> Option<&mut CommentMeta> {
        let part = std::str::from_utf8(part.as_ref()).ok()?;
        self.iter_comment_mut()
            .find(|comment| comment.as_str().contains(part))
    }

    #[inline]
    pub fn get_element<K: AsRef<[u8]>>(
        &self,
        name: K,
    ) -> Option<&ElementMeta> {
        self.get_element_with_data(name).map(|(element, _)| element)
    }

    #[inline]
    pub fn get_element_mut<K: AsRef<[u8]>>(
        &mut self,
        name: K,
    ) -> Option<&mut ElementMeta> {
        self.get_element_with_data_mut(name)
            .map(|(element, _)| element)
    }

    #[inline]
    pub fn get_property<K: AsRef<[u8]>>(
        &self,
        element_name: K,
        property_name: K,
    ) -> Option<&PropertyMeta> {
        self.get_property_with_data(element_name, property_name)
            .map(|(property, _)| property)
    }

    #[inline]
    pub fn get_obj_info<K: AsRef<[u8]>>(
        &self,
        part: K,
    ) -> Option<&ObjInfoMeta> {
        let part = std::str::from_utf8(part.as_ref()).ok()?;
        self.iter_obj_info()
            .find(|obj_info| obj_info.as_str().contains(part))
    }

    #[inline]
    pub fn get_obj_info_mut<K: AsRef<[u8]>>(
        &mut self,
        part: K,
    ) -> Option<&mut ObjInfoMeta> {
        let part = std::str::from_utf8(part.as_ref()).ok()?;
        self.iter_obj_info_mut()
            .find(|obj_info| obj_info.as_str().contains(part))
    }
}

// Filtered accessors with data
impl Object {
    #[inline]
    pub fn get_element_with_data<K: AsRef<[u8]>>(
        &self,
        name: K,
    ) -> Option<(&ElementMeta, &ElementData)> {
        let name = name.as_ref().as_ascii_str().ok()?;
        self.iter_element_with_data()
            .find(|(element, _)| element.name == name)
    }

    #[inline]
    pub fn get_element_with_data_mut<K: AsRef<[u8]>>(
        &mut self,
        name: K,
    ) -> Option<(&mut ElementMeta, &mut ElementData)> {
        let name = name.as_ref().as_ascii_str().ok()?;
        self.iter_element_with_data_mut()
            .find(|(element, _)| element.name == name)
    }

    #[inline]
    pub fn get_property_with_data<K: AsRef<[u8]>>(
        &self,
        element_name: K,
        property_name: K,
    ) -> Option<(&PropertyMeta, &PropertyData)> {
        let element_name = element_name.as_ref().as_ascii_str().ok()?;
        let property_name = property_name.as_ref().as_ascii_str().ok()?;
        self.head
            .iter_element_then_property()
            .zip(self.body.iter())
            .find(|((element, _), _)| element.name == element_name)
            .and_then(|((_, properties), data)| {
                properties
                    .zip(data.iter())
                    .find(|(property, _)| property.name == property_name)
                    .map(|(property, datum)| (property, datum))
            })
    }

    #[inline]
    pub fn get_property_with_dat_mut<K: AsRef<[u8]>>(
        &mut self,
        element_name: K,
        property_name: K,
    ) -> Option<(&PropertyMeta, &mut PropertyData)> {
        let element_name = element_name.as_ref().as_ascii_str().ok()?;
        let property_name = property_name.as_ref().as_ascii_str().ok()?;
        self.head
            .iter_element_then_property()
            .zip(self.body.iter_mut())
            .find(|((element, _), _)| element.name == element_name)
            .and_then(|((_, properties), data)| {
                properties
                    .zip(data.iter_mut())
                    .find(|(property, _)| property.name == property_name)
                    .map(|(property, datum)| (property, datum))
            })
    }
}

// Filtered iterators with data
impl Object {
    #[inline]
    pub fn iter_element_with_data(
        &self
    ) -> impl Iterator<Item = (&ElementMeta, &ElementData)> {
        self.head.iter_element().zip(self.body.iter())
    }

    #[inline]
    pub fn iter_element_with_data_mut(
        &mut self
    ) -> impl Iterator<Item = (&mut ElementMeta, &mut ElementData)> {
        self.head.iter_element_mut().zip(self.body.iter_mut())
    }

    #[inline]
    pub fn iter_property_with_data(
        &self
    ) -> impl Iterator<Item = (&PropertyMeta, &PropertyData)> {
        self.head.iter_property().zip(self.body.iter_data())
    }

    #[inline]
    pub fn iter_property_with_data_mut(
        &mut self
    ) -> impl Iterator<Item = (&mut PropertyMeta, &mut PropertyData)> {
        self.head.iter_property_mut().zip(self.body.iter_data_mut())
    }
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let head = Head::decode(reader)?;
        // TODO
        assert!(!head.is_format_ascii(), "TODO: Decoding on ascii format");

        let should_swap_bytes = !head.get_format().is_native_byte_order();

        let mut body = Body::with_capacity(2);
        head.iter_element_then_property().try_for_each(
            |(element, properties)| -> Result<(), Self::Err> {
                let properties = properties.collect::<Vec<_>>();

                let data = (0..element.size).try_fold(
                    properties
                        .iter()
                        .map(|property| {
                            match &***property {
                                PropertyMetaVariant::Scalar(scalar) => {
                                    let data = ScalarData::with_capacity(
                                        element.size * scalar.size,
                                    );
                                    DataVariant::Scalar(data)
                                },
                                PropertyMetaVariant::List(list) => {
                                    let data = ListData::with_capacity(
                                        element.size * list.value.size,
                                    );
                                    DataVariant::List(data)
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
                                        let mut value = read_bytes(reader, step)?;
                                        if should_swap_bytes {
                                            value.reverse();
                                        }

                                        datum
                                            .as_scalar_mut()
                                            .expect("Unreachable")
                                            .extend(value);
                                    },
                                    PropertyMetaVariant::List(list) => {
                                        let count_step = list.count.size;
                                        let count: usize = match head.get_format() {
                                            BinaryLittleEndian => {
                                                reader.read_uint::<LE>(count_step)
                                            },
                                            Ascii => unreachable!(),
                                            BinaryBigEndian => {
                                                reader.read_uint::<BE>(count_step)
                                            },
                                        }?
                                        .try_into()?;
                                        let value_step = list.value.size;
                                        let value_steps = count * value_step;
                                        let mut value = read_bytes(reader, value_steps)?;
                                        if should_swap_bytes {
                                            value
                                                .chunks_exact_mut(value_step)
                                                .for_each(|value| value.reverse());
                                        }

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
                body.push(data);
                Ok(())
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
        // TODO
        assert!(
            !self.head.is_format_ascii(),
            "TODO: Encoding on ascii format"
        );

        let should_swap_bytes = !self.head.get_format().is_native_byte_order();

        self.head
            .iter_element_then_property()
            .zip(self.body.iter())
            .try_for_each(|((element, properties), data)| -> Result<(), Self::Err> {
                let properties = properties.collect::<Vec<_>>();
                (0..element.size).try_for_each(
                    |element_index| -> Result<(), Self::Err> {
                        properties.iter().zip(data.iter()).try_for_each(
                            |(property, datum)| -> Result<(), Self::Err> {
                                match &***property {
                                    PropertyMetaVariant::Scalar(scalar) => {
                                        let datum =
                                            datum.as_scalar().expect("Unreachable");
                                        let step = scalar.size;
                                        let offset = element_index * step;
                                        let value = datum
                                            .get(offset..offset + step)
                                            .ok_or_else(|| {
                                                Error::OutOfBound(
                                                    offset + step,
                                                    datum.len(),
                                                )
                                            })?;

                                        if should_swap_bytes {
                                            let value = &mut value.to_owned();
                                            value.reverse();
                                            writer.write_all(value)?;
                                        } else {
                                            writer.write_all(value)?;
                                        }
                                    },
                                    PropertyMetaVariant::List(list) => {
                                        let datum = datum.as_list().expect("Unreachable");
                                        let value = datum.get(element_index).ok_or_else(
                                            || {
                                                Error::OutOfBound(
                                                    element_index,
                                                    datum.len(),
                                                )
                                            },
                                        )?;
                                        let value_step = list.value.size;
                                        let count = (value.len() / value_step) as u64;
                                        let count_step = list.count.size;
                                        match self.head.get_format() {
                                            BinaryLittleEndian => {
                                                writer.write_uint::<LE>(
                                                    count, count_step,
                                                )?;
                                            },
                                            Ascii => unreachable!(),
                                            BinaryBigEndian => {
                                                writer.write_uint::<BE>(
                                                    count, count_step,
                                                )?;
                                            },
                                        }

                                        if should_swap_bytes {
                                            let value = &mut value.to_owned();
                                            value
                                                .chunks_exact_mut(value_step)
                                                .for_each(|value| value.reverse());
                                            writer.write_all(value)?;
                                        } else {
                                            writer.write_all(value)?;
                                        }
                                    },
                                };
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                Ok(())
            })?;
        Ok(())
    }
}

impl ops::Deref for Object {
    type Target = Head;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

impl ops::DerefMut for Object {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.head
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_byte_order_on_custom_method() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let target = include_bytes!(
            "../../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
        );
        assert_ne!(source[0x18f..], target[0x18b..]);

        let reader = &mut Cursor::new(source);
        let mut output = Object::decode(reader).unwrap();

        output.set_format(BinaryBigEndian);

        let mut writer = Cursor::new(vec![]);
        output.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        output.iter().zip(target.iter()).enumerate().for_each(
            |(index, (output, target))| {
                assert_eq!(output, target, "index: {}", index);
            },
        );
    }

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

        let target = 4;
        let output = object.head.iter().filter(|meta| meta.is_element()).count();
        assert_eq!(output, target);

        let target = 11;
        let output = object.head.iter().filter(|meta| meta.is_property()).count();
        assert_eq!(output, target);

        let target = 11;
        let output = object.body.iter_data().count();
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
