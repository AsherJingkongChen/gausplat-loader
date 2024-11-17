pub mod id;

pub use id::*;
pub use indexmap::IndexMap;

pub struct Object {}

// TODO: Relations (Bindings)
//
// - Head <-> Body
// - HeadBlock { id }
// - HeadBlock::Element <-> BodyBlock { id, data }
// - HeadBlock::Property <-> DataBlock { id, inner }

// TODO: Access patterns
//
// get(e).len()
// get(e).get(p).as (Planar)
// get(e).iter().as (Packed)

// TODO: Byte-order aware casting: Conversion
