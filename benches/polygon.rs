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
    fn polygon(bencher: Bencher) {
        use gausplat_loader::source::polygon::{Decoder, Object};

        bencher.with_inputs(data::get()).bench_local_refs(|v| {
            let mut reader = Cursor::new(v);
            let object = Object::decode(&mut reader).unwrap();
            let flags = &[
                object.get_property_as::<f32>("vertex", "x"),
                object.get_property_as::<f32>("vertex", "y"),
                object.get_property_as::<f32>("vertex", "z"),
            ]
            .map(|v| v.unwrap().1.get(0).copied().unwrap());
            assert_eq!(data::FLAGS, flags);
            object
        })
    }

    pub mod data {
        pub use super::*;

        pub const ELEMENT_COUNT: usize = 1 << 20;
        pub const ELEMENT_SIZE: usize = ELEMENT_COUNT * (3 * 4);
        pub const FLAGS: &[f32; 3] = &[0.123456, -7.89012, 3.456789];
        pub const HEADER: &str = "\
            ply\n\
            format binary_little_endian 1.0\n\
            element vertex 1048576\n\
            property float x\n\
            property float y\n\
            property float z\n\
            end_header\n\
        ";

        pub fn get() -> impl FnMut() -> Vec<u8> {
            move || {
                let mut object = format!(
                    "ply\n\
                    format binary_little_endian 1.0\n\
                    element vertex {ELEMENT_COUNT}\n\
                    property float x\n\
                    property float y\n\
                    property float z\n\
                    end_header\n"
                )
                .into_bytes();

                let payload = {
                    let mut data = random::get_vec_u8(ELEMENT_SIZE)();
                    let view =
                        bytemuck::try_cast_slice_mut::<u8, f32>(&mut data).unwrap();
                    view[0..3].copy_from_slice(FLAGS);
                    data
                };
                object.extend(payload);
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
