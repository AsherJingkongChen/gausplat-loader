pub use divan::Bencher;
pub use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
pub use std::io::{Cursor, Read};

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
            let mut reader = Cursor::new(v);
            let parser = Parser::<DefaultElement>::new();
            let object = parser.read_ply(&mut reader).unwrap();
            let element = &object.payload["vertex"][0];
            let flags = &[&element["x"], &element["y"], &element["z"]].map(|v| match v {
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
            let mut reader = Cursor::new(v);
            let object = Object::decode(&mut reader).unwrap();
            let element = object.elem("vertex").unwrap();
            let flags = &[element.prop("x"), element.prop("y"), element.prop("z")]
                .map(|v| v.unwrap().cast::<f32>().unwrap()[0]);
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    mod data {
        pub use super::*;

        pub const ELEMENT_COUNT: usize = 500500;
        pub const ELEMENT_SIZE: usize = ELEMENT_COUNT * (3 * 4);
        pub const FLAGS: &[f32; 3] = &[0.123456, -7.89012, 3.456789];
        pub const HEADER: &[u8] = b"\
            ply\n\
            format binary_little_endian 1.0\n\
            element vertex 500500\n\
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
pub mod splat_decode {
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
            let element = &object.payload["splat"][0];
            let flags = &[
                &element["red"],
                &element["green"],
                &element["blue"],
                &element["alpha"],
            ]
            .map(|v| match v {
                UChar(v) => *v,
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
            let mut reader = Cursor::new(v);
            let object = Object::decode(&mut reader).unwrap();
            let element = object.elem("splat").unwrap();
            let flags = &[
                element.prop("red"),
                element.prop("green"),
                element.prop("blue"),
                element.prop("alpha"),
            ]
            .map(|v| v.unwrap().cast::<u8>().unwrap()[0]);
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    mod data {
        pub use super::*;

        pub const ELEMENT_COUNT: usize = 500500;
        pub const ELEMENT_SIZE: usize = ELEMENT_COUNT * ((2) * 4 + (4) * 1);
        pub const FLAGS: &[u8; 4] = b"RGBA";
        pub const HEADER: &[u8] = b"\
            ply\n\
            format binary_little_endian 1.0\n\
            element splat 500500\n\
            property uchar red\n\
            property uchar green\n\
            property uchar blue\n\
            property uchar alpha\n\
            property float x\n\
            property float y\n\
            end_header\n\
        ";

        pub fn get() -> impl FnMut() -> Vec<u8> {
            move || {
                let mut object = HEADER.to_owned();
                let mut data = random::get_vec_u8(ELEMENT_SIZE)();
                let view = bytemuck::try_cast_slice_mut::<u8, u8>(&mut data).unwrap();
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
