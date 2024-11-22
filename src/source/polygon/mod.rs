//! `polygon` can read and write polygon files (PLY).
//!
//! # Examples
//!
//! **Note:** Click triangle to view content.
//!
//! <details>
//! <summary>
//!     <strong><code>another-cube.greg-turk.ascii.ply</code>:</strong>
//! </summary>
//! <pre class=language-plaintext>
#![doc = include_str!("../../../examples/data/polygon/another-cube.greg-turk.ascii.ply")]
//! </pre>
//! </details>
#![doc = include_str!("SUPPLEMENT.md")]
#![doc = include_str!("LICENSE.md")]

pub mod block;
pub mod format;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use block::*;
pub use format::*;

use crate::function::{
    decode::{
        is_space, read_byte_after, read_bytes, read_bytes_before,
        read_bytes_before_newline, read_bytes_const, take_newline,
    },
    encode::{NEWLINE, SPACE},
};
use std::{
    io::{Read, Write},
    ops,
};
use BlockVariant::*;
use PropertyBlockVariant::*;

// TODO: Accessors, Removers, Iterators, and Extenders

/// ## Syntax
///
/// ```plaintext
/// <object> :=
///     | <start_of_header> <format-block> [{<block>}] <end_of_header>
///
/// <start_of_header> :=
///     | "ply" <newline>
///
/// <block> :=
///     | "comment " <comment-block>
///     | "element " <element-block>
///     | "obj_info " <obj_info-block>
///     | "property " <property-block>
///
/// <end_of_header> :=
///     | "end_header" <newline>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`Block`]
/// - [`Format`]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Object {
    pub blocks: Vec<Block>,
    pub format: Format,
}

impl Object {
    pub const KEYWORDS: &[&str; 5] = &[
        "comment ",
        "element ",
        "end_header",
        "property ",
        "obj_info ",
    ];
    pub const SIGNATURE: &[u8; 3] = b"ply";

    #[inline]
    pub fn element_and_property_indices(&self) -> Vec<(usize, Vec<usize>)> {
        self.iter()
            .enumerate()
            .filter(|(_, block)| block.is_element())
            .map(|(index, _)| {
                (
                    index,
                    self.iter()
                        .enumerate()
                        .skip(index + 1)
                        .take_while(|(_, block)| block.is_property())
                        .map(|(index, _)| index)
                        .collect(),
                )
            })
            .collect()
    }
}

impl Decoder for Object {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        const CAPACITY_BLOCKS: usize = 32;

        // Decoding the signature.

        if read_bytes_const(reader)? != *Self::SIGNATURE {
            // SAFETY: This is a UTF-8 string literal.
            Err(Error::MissingToken(unsafe {
                String::from_utf8(Self::SIGNATURE.into()).unwrap_unchecked()
            }))?;
        }
        take_newline(reader)?;

        // Decoding the format.

        let mut blocks = Vec::with_capacity(CAPACITY_BLOCKS);
        let format = Format::decode(reader)?;

        // Decoding all blocks until the end of header.

        let mut had_element = false;

        loop {
            let keyword_prefix = read_bytes_const(reader)?;
            let variant = match &keyword_prefix {
                b"pr" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"operty " => {
                            if had_element {
                                Ok(Property(PropertyBlock::decode(reader)?))
                            } else {
                                Err(Error::MissingToken("element ".into()))?
                            }
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"el" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"ement " => {
                            had_element = true;
                            Ok(Element(ElementBlock::decode(reader)?))
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"en" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"d_header" => {
                            take_newline(reader)?;
                            break;
                        },
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"co" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"mment " => Ok(Comment(CommentBlock::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                b"ob" => {
                    let keyword_suffix = read_bytes_const(reader)?;
                    match &keyword_suffix {
                        b"j_info " => Ok(ObjInfo(ObjInfoBlock::decode(reader)?)),
                        _ => Err(keyword_suffix.into()),
                    }
                },
                _ => {
                    let keyword_suffix = read_bytes_before(reader, is_space, 8)?;
                    Err(keyword_suffix.into())
                },
            }
            .map_err(|keyword_suffix: Box<[u8]>| {
                let keyword = &[&keyword_prefix[..], &keyword_suffix].concat();
                Error::InvalidPolygonKeyword(
                    String::from_utf8_lossy(keyword).into_owned(),
                )
            })?;

            blocks.push(Block { variant });
        }

        // Decoding all property data until the end of object.

        let mut object = Object { blocks, format };

        for (element_index, property_indices) in object.element_and_property_indices() {
            // Allocating the property data for the current element,

            // SAFETY: The index is in bounds and the block is an element.
            let element_size = unsafe {
                object
                    .get_unchecked(element_index)
                    .as_element()
                    .unwrap_unchecked()
                    .size
            };
            for &property_index in &property_indices {
                // SAFETY: The index is in bounds and the block is a property.
                let property = unsafe {
                    object
                        .get_unchecked_mut(property_index)
                        .as_property_mut()
                        .unwrap_unchecked()
                };
                match &mut **property {
                    Scalar(scalar) => {
                        scalar.data.reserve(element_size * scalar.step);
                    },
                    List(list) => {
                        list.data.reserve(element_size * list.value.step);
                    },
                }
            }

            // Decoding the property data for the current element.

            let format = *object.format;

            for _element_index in 0..element_size {
                for &property_index in &property_indices {
                    // SAFETY: The index is in bounds.
                    let property = unsafe {
                        object
                            .get_unchecked_mut(property_index)
                            .as_property_mut()
                            .unwrap_unchecked()
                    };
                    match &mut **property {
                        Scalar(ScalarPropertyBlock { data, info }) => {
                            match format {
                                Ascii => {
                                    // TODO: Decoding scalar for ascii format.
                                    todo!()
                                },
                                _ => {
                                    // Decoding the scalar property datum in binary format.

                                    let mut datum = read_bytes(reader, info.step)?;
                                    if !format.is_binary_native_endian() {
                                        datum.reverse();
                                    }
                                    data.extend(datum);
                                },
                            }
                        },
                        List(ListPropertyBlock { data, info }) => match format {
                            Ascii => {
                                // TODO: Decoding list for ascii format.
                                todo!()
                            },
                            _ => {
                                // Decoding the list property datum in binary format.

                                let count = {
                                    let mut bytes = read_bytes(reader, info.count.step)?;
                                    if !format.is_binary_little_endian() {
                                        bytes.reverse();
                                    }
                                    bytes.resize(size_of::<usize>(), 0);

                                    // SAFETY: The bytes are of the exact size.
                                    usize::from_le_bytes(unsafe {
                                        bytes.try_into().unwrap_unchecked()
                                    })
                                };
                                let mut datum =
                                    read_bytes(reader, count * info.value.step)?;
                                if !format.is_binary_native_endian() {
                                    datum
                                        .chunks_exact_mut(info.value.step)
                                        .for_each(|datum| datum.reverse());
                                }
                                data.push(datum);
                            },
                        },
                    }
                }
            }
        }

        Ok(object)
    }
}

impl Encoder for Object {
    type Err = Error;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        // Encoding the signature.

        writer.write_all(b"ply")?;
        writer.write_all(NEWLINE)?;

        // Encoding the format.

        self.format.encode(writer)?;

        // Encoding all blocks before the end of header.

        let mut had_element = false;

        for block in self.iter() {
            match &**block {
                Property(block) => {
                    if had_element {
                        writer.write_all(b"property ")?;
                        block.encode(writer)
                    } else {
                        Err(Error::MissingToken("element ".into()))?
                    }
                },
                Element(block) => {
                    had_element = true;
                    writer.write_all(b"element ")?;
                    block.encode(writer)
                },
                Comment(block) => {
                    writer.write_all(b"comment ")?;
                    block.encode(writer)
                },
                ObjInfo(block) => {
                    writer.write_all(b"obj_info ")?;
                    block.encode(writer)
                },
            }?;
        }

        // Encoding the end of header.

        writer.write_all(b"end_header")?;
        writer.write_all(NEWLINE)?;

        // Encoding all property data until the end of object.

        let format = *self.format;

        for (element_index, property_indices) in self.element_and_property_indices() {
            // SAFETY: The index is in bounds and the block is an element.
            let element_size = {
                self.get(element_index)
                    .expect("Unreachable")
                    .as_element()
                    .expect("Unreachable")
                    .size
            };
            for element_index in 0..element_size {
                for &property_index in &property_indices {
                    // SAFETY: The index is in bounds.
                    let property = {
                        self.get(property_index)
                            .expect("Unreachable")
                            .as_property()
                            .expect("Unreachable")
                    };
                    match &**property {
                        Scalar(ScalarPropertyBlock { data, info }) => {
                            // Reading the scalar property datum.

                            let start = element_index * info.step;
                            let end = start + info.step;
                            let datum = data
                                .get(start..end)
                                .ok_or_else(|| Error::OutOfBounds(end - 1, data.len()))?;

                            match format {
                                Ascii => {
                                    // TODO: Encoding scalar for ascii format.
                                    todo!()
                                },
                                _ => {
                                    // Encoding the scalar property datum in binary format.

                                    if format.is_binary_native_endian() {
                                        writer.write_all(datum)?;
                                    } else {
                                        let datum = &mut datum.to_owned();
                                        datum.reverse();
                                        writer.write_all(datum)?;
                                    }
                                },
                            }
                        },
                        List(ListPropertyBlock { data, info }) => {
                            // Reading the list property datum.

                            let datum = data.get(element_index).ok_or_else(|| {
                                Error::OutOfBounds(element_index, data.len())
                            })?;
                            if datum.len() % info.value.step != 0 {
                                Err(Error::MisalignedBytes(data.len(), info.value.step))?;
                            }

                            let count = (datum.len() / info.value.step).to_le_bytes();
                            let count =
                                count.get(..info.count.step).ok_or_else(|| {
                                    Error::OutOfBounds(
                                        info.count.step,
                                        size_of_val(&count),
                                    )
                                })?;

                            match format {
                                Ascii => {
                                    // TODO: Encoding list for ascii format.
                                    todo!()
                                },
                                _ => {
                                    // Encoding the list property datum in binary format.

                                    if format.is_binary_native_endian() {
                                        writer.write_all(count)?;
                                        writer.write_all(datum)?;
                                    } else {
                                        let count = &mut count.to_owned();
                                        let datum = &mut datum.to_owned();
                                        count.reverse();
                                        datum
                                            .chunks_exact_mut(info.value.step)
                                            .for_each(|datum| datum.reverse());
                                        writer.write_all(count)?;
                                        writer.write_all(datum)?;
                                    }
                                },
                            }
                        },
                    }
                }
            }
        }
        Ok(())
    }
}

impl ops::Deref for Object {
    type Target = Vec<Block>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl ops::DerefMut for Object {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.blocks
    }
}

macro_rules! impl_variant_matchers {
    ($subject:ident, $( $variant:ident ),* ) => {
        paste::paste! {
            impl [<$subject Variant>] {
                $(
                    #[inline]
                    pub const fn [<as_ $variant:snake>](&self) -> Option<&[<$variant $subject>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }

                    #[inline]
                    pub fn [<as_ $variant:snake _mut>](&mut self) -> Option<&mut [<$variant $subject>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }

                    #[inline]
                    pub const fn [<is_ $variant:snake>](&self) -> bool {
                        matches!(self, Self::$variant(_))
                    }

                    #[inline]
                    pub fn [<into_ $variant:snake>](self) -> Option<[<$variant $subject>]> {
                        match self {
                            Self::$variant(data) => Some(data),
                            _ => None,
                        }
                    }
                )*
            }

            impl std::ops::Deref for $subject {
                type Target = [<$subject Variant>];

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.variant
                }
            }

            impl std::ops::DerefMut for $subject {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.variant
                }
            }
        }
    };
}
use impl_variant_matchers;

#[cfg(test)]
mod tests {
    #[test]
    fn decode_and_encode() {
        use super::*;
        use std::io::Cursor;

        let source =
            include_bytes!("../../../examples/data/polygon/valid-keyword.ascii.ply");
        let reader = &mut Cursor::new(source);
        let output = Object::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        output.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_misplaced_property() {
        use super::*;
        use std::io::Cursor;

        let source =
            include_bytes!("../../../examples/data/polygon/misplaced-property.ascii.ply");

        let reader = &mut Cursor::new(source);
        Object::decode(reader).unwrap_err();
    }

    #[test]
    fn decode_on_empty_head() {
        use super::*;
        use std::io::{Cursor, ErrorKind};

        let source =
            include_bytes!("../../../examples/data/polygon/empty-head.ascii.ply");

        let reader = &mut Cursor::new(source);
        let target = Object::default();
        let output = Object::decode(reader).unwrap();
        assert_eq!(output, target);

        let target = ErrorKind::UnexpectedEof;
        let output = reader.read_exact(&mut [0; 1]).unwrap_err().kind();
        assert_eq!(output, target);

        let reader = &mut Cursor::new(&source[..source.len() - 1]);
        Object::decode(reader).unwrap_err();

        let reader =
            &mut Cursor::new([&source[..source.len() - 1], b" not newline"].concat());
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply\nend_header\n");
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply");
        Object::decode(reader).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_keyword() {
        use super::*;
        use std::io::Cursor;

        let header = &b"ply\nformat ascii 1.0\n"[..];

        let source = &mut Cursor::new([header, b"invalid\n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"invalid \n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"commemt \n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"elenemt \n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"end_header \n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"end_haeder\n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"obj-info \n"].concat());
        Object::decode(source).unwrap_err();

        let source = &mut Cursor::new([header, b"proprety \n"].concat());
        Object::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_signature() {
        use super::*;
        use std::io::Cursor;

        let source =
            include_bytes!("../../../examples/data/polygon/empty-head.ascii.ply");

        let reader = &mut Cursor::new([b"plh", &source[3..]].concat());
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"  ply\n");
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply  \n");
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new(b"ply  ");
        Object::decode(reader).unwrap_err();

        let reader = &mut Cursor::new([b"ply\r\n", &source[4..]].concat());
        let target = Object::default();
        let output = Object::decode(reader).unwrap();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_on_le_and_encode_on_be() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let target = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
        );
        assert_ne!(source[0x18f..], target[0x18b..]);

        let reader = &mut Cursor::new(source);
        let mut output = Object::decode(reader).unwrap();

        *output.format = FormatVariant::BinaryBigEndian;

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
            "../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
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
    fn decode_on_binary_be_incomplete() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-be.ply"
        );

        for index in 0..(source.len() - 1) {
            let reader = &mut Cursor::new(&source[..index]);
            Object::decode(reader).unwrap_err();
        }
    }

    #[test]
    fn decode_and_encode_on_binary_le() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
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

        let source =
            include_bytes!("../../../examples/data/polygon/empty-element.binary-le.ply");
        let reader = &mut Cursor::new(source);
        let object = Object::decode(reader).unwrap();

        let target = 4;
        let output = object.iter().filter(|block| block.is_element()).count();
        assert_eq!(output, target);

        let target = 11;
        let output = object.iter().filter(|block| block.is_property()).count();
        assert_eq!(output, target);

        let target = 6 * 11;
        let output = object
            .iter()
            .filter_map(|block| Some(block.as_property()?.as_scalar()?.data.len()))
            .sum::<usize>();
        assert_eq!(output, target);
    }

    #[test]
    fn decode_and_encode_on_empty_element() {
        use super::*;
        use std::io::Cursor;

        let source =
            include_bytes!("../../../examples/data/polygon/empty-element.binary-le.ply");
        let reader = &mut Cursor::new(source);
        let object = Object::decode(reader).unwrap();

        let mut writer = Cursor::new(vec![]);
        let target = source;
        object.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        assert_eq!(output, target);
    }

    #[test]
    fn decode_and_encode_on_zero_filled() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let target = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.zeros.binary-le.ply"
        );

        let reader = &mut Cursor::new(source);
        let mut object = Object::decode(reader).unwrap();

        object
            .iter_mut()
            .filter_map(|block| block.as_property_mut())
            .for_each(|property| match &mut **property {
                Scalar(scalar) => {
                    scalar.data.fill(0);
                },
                List(list) => {
                    list.data.iter_mut().for_each(|datum| datum.fill(0));
                },
            });

        let mut writer = Cursor::new(vec![]);
        object.encode(&mut writer).unwrap();
        let output = writer.into_inner();

        output.iter().zip(target.iter()).enumerate().for_each(
            |(index, (output, target))| {
                assert_eq!(output, target, "index: {}", index);
            },
        );
    }

    #[test]
    fn decode_and_cast_on_triangle() {
        // TODO: Implement multi-byte casting.
        // use super::*;
        // use std::io::Cursor;

        // let target =
        //     include_bytes!("../../../examples/data/polygon/triangle.binary-le.ply");
        // let reader = &mut Cursor::new(target);
        // let mut object = Object::decode(reader).unwrap();

        // let (block, data) = object.remove_property_with_data("vertex", "x").unwrap();
        // let data = data.into_inner().into_scalar().unwrap();

        // assert_eq!(block.name, "x");
        // assert_eq!(data.len(), 12);
        // assert_eq!(
        //     data.cast::<f32>().unwrap(),
        //     [0.120001904666423798, 0.0, -0.23999999463558197]
        // );

        // let (block, data) = object.get_property_with_data("vertex", "y").unwrap();
        // let data = data.as_scalar().unwrap();

        // assert_eq!(block.name, "y");
        // assert_eq!(data.len(), 12);
        // assert_eq!(
        //     data.cast::<f32>().unwrap(),
        //     [-0.119999997317790985, 0.0, 1.17549435082228751e-38]
        // );
    }

    #[test]
    fn decode_and_encode_on_duplicate_format() {
        use super::*;
        use std::io::Cursor;

        let source =
            include_bytes!("../../../examples/data/polygon/duplicate-format.ascii.ply");
        let reader = &mut Cursor::new(source);
        Object::decode(reader).unwrap_err();
    }

    #[test]
    fn encode_on_misaligned_lists() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let mut output = Object::decode(reader).unwrap();

        output
            .iter_mut()
            .filter_map(|block| Some(&mut block.as_property_mut()?.as_list_mut()?.data))
            .for_each(|data| {
                data.iter_mut().for_each(|datum| datum.push(1));
            });

        let mut writer = Cursor::new(vec![]);
        let target = true;
        let output = matches!(
            output.encode(&mut writer).unwrap_err(),
            Error::MisalignedBytes(_, _)
        );
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_out_of_bound() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let mut output = Object::decode(reader).unwrap();

        output
            .iter_mut()
            .filter_map(|block| Some(&mut block.as_element_mut()?.size))
            .for_each(|size| {
                *size += 1;
            });

        let mut writer = Cursor::new(vec![]);
        let target = true;
        let output = matches!(
            output.encode(&mut writer).unwrap_err(),
            Error::OutOfBounds(_, _)
        );
        assert_eq!(output, target);
    }

    #[test]
    fn encode_on_misplaced_property() {
        use super::*;
        use std::io::Cursor;

        let mut output = Object {
            blocks: vec![
                CommentBlock::new("This example polygon file shows a misplaced property after cleaning.").unwrap().into(),
                ElementBlock::decode(&mut Cursor::new(b"point 0\n")).unwrap().into(),
                PropertyBlock::decode(&mut Cursor::new(b"float y\n")).unwrap().into(),
            ],
            format: Ascii.into(),
        };
        output.insert(1, PropertyBlock::new("x", FLOAT.to_owned()).unwrap().into());

        let mut writer = Cursor::new(vec![]);
        output.encode(&mut writer).unwrap_err();
    }

    #[test]
    fn encode_on_empty_slice() {
        use super::*;

        Object::default().encode(&mut &mut [0; 25][..]).unwrap_err();
    }

    #[test]
    fn remove_comment_on_all() {
        use super::*;
        use std::io::Cursor;

        let source = include_bytes!(
            "../../../examples/data/polygon/another-cube.greg-turk.binary-le.ply"
        );
        let reader = &mut Cursor::new(source);
        let mut object = Object::decode(reader).unwrap();

        let target = 2;
        let output = object.iter().filter(|block| block.is_comment()).count();
        assert_eq!(output, target);

        object.retain(|block| !block.is_comment());

        let target = 0;
        let output = object.iter().filter(|block| block.is_comment()).count();
        assert_eq!(output, target);
    }
}
