pub use divan::Bencher;
pub use std::io::{BufReader, Cursor, Read};

fn main() {
    divan::main();
}

pub mod vertex {
    pub use super::*;

    pub const ELEMENT_COUNT: usize = 1 << 23;

    #[divan::bench(sample_count = 10, sample_size = 2)]
    fn polygon_decode(bencher: Bencher) {
        use gausplat_loader::source::polygon::{Decoder, Object};

        bencher
            .with_inputs(data::get_random_element_vertex(ELEMENT_COUNT))
            .bench_local_values(|v| {
                let mut reader = Cursor::new(v);
                Object::decode(&mut reader).unwrap()
            })
    }
}

pub mod data {
    pub use super::*;
    pub use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

    pub fn get_random_vec_u8(size: usize) -> impl FnMut() -> Vec<u8> {
        move || {
            StdRng::seed_from_u64(0)
                .sample_iter(Uniform::new_inclusive(u8::MIN, u8::MAX))
                .take(size)
                .collect()
        }
    }

    pub fn get_random_element_vertex(count: usize) -> impl FnMut() -> Vec<u8> {
        move || {
            let mut object = format!(
                "ply\n\
                format binary_little_endian 1.0\n\
                element vertex {count}\n\
                property float x\n\
                property float y\n\
                property float z\n\
                end_header\n"
            )
            .into_bytes();
            let size = count * 3 * 4;
            object.reserve(size);
            object.extend(get_random_vec_u8(size)());
            object
        }
    }
}
