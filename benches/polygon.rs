pub use divan::Bencher;
pub use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
pub use std::io::{BufReader, Cursor, Read};

fn main() {
    divan::main();
}

/// ## Tasks
///
/// 1. Parse the header.
/// 2. Read the payload.
/// 3. Ensure that user can access the data.
pub mod vertex_decode {
    pub use super::*;

    #[divan::bench(sample_count = 25, sample_size = 2)]
    fn ply_rs(bencher: Bencher) {
        use ply_rs::{
            parser::Parser,
            ply::{DefaultElement, Property::*},
        };

        bencher.with_inputs(data::get()).bench_local_refs(|v| {
            let mut reader = BufReader::new(Cursor::new(v));
            let parser = Parser::<DefaultElement>::new();
            let object = parser.read_ply(&mut reader).unwrap();
            let flags = &[
                &object.payload["vertex"][0]["x"],
                &object.payload["vertex"][0]["y"],
                &object.payload["vertex"][0]["z"],
            ]
            .map(|v| match v {
                Float(v) => *v,
                other => panic!("{:?}", other),
            });
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    #[divan::bench(sample_count = 25, sample_size = 2)]
    fn polygon(bencher: Bencher) {
        use gausplat_loader::source::polygon::{Decoder, Object};

        bencher.with_inputs(data::get()).bench_local_refs(|v| {
            // NOTE: Object::decode has an internal buffer.
            let mut reader = Cursor::new(v);
            let object = Object::decode(&mut reader).unwrap();
            let flags = &[
                object.get_property_as::<f32>("vertex", "x"),
                object.get_property_as::<f32>("vertex", "y"),
                object.get_property_as::<f32>("vertex", "z"),
            ]
            .map(|v| v.unwrap().1[0]);
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    mod data {
        pub use super::*;

        pub const ELEMENT_COUNT: usize = 1001001;
        pub const ELEMENT_SIZE: usize = ELEMENT_COUNT * (3 * 4);
        pub const FLAGS: &[f32; 3] = &[0.123456, -7.89012, 3.456789];
        pub const HEADER: &[u8] = b"\
            ply\n\
            format binary_little_endian 1.0\n\
            element vertex 1001001\n\
            property float x\n\
            property float y\n\
            property float z\n\
            end_header\n\
        ";

        pub fn get() -> impl FnMut() -> Vec<u8> {
            move || {
                let mut object = HEADER.to_owned();
                let mut data = random::get_vec_u8(ELEMENT_SIZE)();
                let view = bytemuck::try_cast_slice_mut::<u8, f32>(&mut data).unwrap();
                view[0..FLAGS.len()].copy_from_slice(FLAGS);
                object.extend(data);
                object
            }
        }
    }
}

/// ## Tasks
///
/// 1. Parse the header.
/// 2. Read the payload.
/// 3. Ensure that user can access the data.
pub mod splats_decode {
    pub use super::*;

    #[divan::bench(sample_count = 25, sample_size = 2)]
    fn ply_rs(bencher: Bencher) {
        use ply_rs::{
            parser::Parser,
            ply::{DefaultElement, Property::*},
        };

        bencher.with_inputs(data::get()).bench_local_refs(|v| {
            let mut reader = Cursor::new(v);
            let parser = Parser::<DefaultElement>::new();
            let object = parser.read_ply(&mut reader).unwrap();
            let flags = &[
                &object.payload["vertex"][0]["x"],
                &object.payload["vertex"][0]["y"],
                &object.payload["vertex"][0]["z"],
                &object.payload["vertex"][0]["nx"],
                &object.payload["vertex"][0]["ny"],
                &object.payload["vertex"][0]["nz"],
            ]
            .map(|v| match v {
                Float(v) => *v,
                other => panic!("{:?}", other),
            });
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    #[divan::bench(sample_count = 25, sample_size = 2)]
    fn polygon(bencher: Bencher) {
        use gausplat_loader::source::polygon::{Decoder, Object};

        bencher.with_inputs(data::get()).bench_local_refs(|v| {
            // NOTE: Object::decode has an internal buffer.
            let mut reader = Cursor::new(v);
            let object = Object::decode(&mut reader).unwrap();
            let flags = &[
                object.get_property_as::<f32>("vertex", "x"),
                object.get_property_as::<f32>("vertex", "y"),
                object.get_property_as::<f32>("vertex", "z"),
                object.get_property_as::<f32>("vertex", "nx"),
                object.get_property_as::<f32>("vertex", "ny"),
                object.get_property_as::<f32>("vertex", "nz"),
            ]
            .map(|v| v.unwrap().1[0]);
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    mod data {
        pub use super::*;

        pub const ELEMENT_COUNT: usize = 100100;
        pub const ELEMENT_SIZE: usize = ELEMENT_COUNT * (62 * 4);
        pub const FLAGS: &[f32; 6] = &[0.123456, -7.89012, 3.456789, 0.0, 0.0, 0.0];
        pub const HEADER: &[u8] = b"\
            ply\n\
            format binary_little_endian 1.0\n\
            element vertex 100100\n\
            property float x\n\
            property float y\n\
            property float z\n\
            property float nx\n\
            property float ny\n\
            property float nz\n\
            property float f_dc_0\n\
            property float f_dc_1\n\
            property float f_dc_2\n\
            property float f_rest_0\n\
            property float f_rest_1\n\
            property float f_rest_2\n\
            property float f_rest_3\n\
            property float f_rest_4\n\
            property float f_rest_5\n\
            property float f_rest_6\n\
            property float f_rest_7\n\
            property float f_rest_8\n\
            property float f_rest_9\n\
            property float f_rest_10\n\
            property float f_rest_11\n\
            property float f_rest_12\n\
            property float f_rest_13\n\
            property float f_rest_14\n\
            property float f_rest_15\n\
            property float f_rest_16\n\
            property float f_rest_17\n\
            property float f_rest_18\n\
            property float f_rest_19\n\
            property float f_rest_20\n\
            property float f_rest_21\n\
            property float f_rest_22\n\
            property float f_rest_23\n\
            property float f_rest_24\n\
            property float f_rest_25\n\
            property float f_rest_26\n\
            property float f_rest_27\n\
            property float f_rest_28\n\
            property float f_rest_29\n\
            property float f_rest_30\n\
            property float f_rest_31\n\
            property float f_rest_32\n\
            property float f_rest_33\n\
            property float f_rest_34\n\
            property float f_rest_35\n\
            property float f_rest_36\n\
            property float f_rest_37\n\
            property float f_rest_38\n\
            property float f_rest_39\n\
            property float f_rest_40\n\
            property float f_rest_41\n\
            property float f_rest_42\n\
            property float f_rest_43\n\
            property float f_rest_44\n\
            property float opacity\n\
            property float scale_0\n\
            property float scale_1\n\
            property float scale_2\n\
            property float rot_0\n\
            property float rot_1\n\
            property float rot_2\n\
            property float rot_3\n\
            end_header\n\
        ";

        pub fn get() -> impl FnMut() -> Vec<u8> {
            move || {
                let mut object = HEADER.to_owned();
                let mut data = random::get_vec_u8(ELEMENT_SIZE)();
                let view = bytemuck::try_cast_slice_mut::<u8, f32>(&mut data).unwrap();
                view[0..FLAGS.len()].copy_from_slice(FLAGS);
                object.extend(data);
                object
            }
        }
    }
}

pub mod random {
    pub use super::*;

    pub fn get_vec_u8(size: usize) -> impl FnMut() -> Vec<u8> {
        move || {
            StdRng::seed_from_u64(0)
                .sample_iter(Uniform::new_inclusive(u8::MIN, u8::MAX))
                .take(size)
                .collect()
        }
    }
}
