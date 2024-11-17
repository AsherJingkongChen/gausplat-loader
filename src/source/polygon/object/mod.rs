pub mod id;

pub use id::*;
pub use indexmap::IndexMap;

pub struct Object {}

// TODO: Relations (Bindings)
//
// - Head <-> Body
// - HeadBlock { id, variant } <-> BodyBlock { id, variant }
//     - head::ListPropertyBlock <-> body::ListPropertyBlock
//     - head::ScalarPropertyBlock <-> body::ScalarPropertyBlock
