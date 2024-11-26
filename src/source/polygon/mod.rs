//! The module `polygon` can read and write PLY polygon files (`*.ply`).
//!
//! # Examples
//!
//! **Note:** Click triangle to view content.
//!
//! ----
//!
//! <details>
//! <summary>
//!     <strong><code>another-cube.greg-turk.ascii.ply</code>:</strong>
//! </summary>
//! <pre class=language-plaintext>
#![doc = include_str!("../../../examples/data/polygon/another-cube.greg-turk.ascii.ply")]
//! </pre>
//! </details>
//!
//! ----
//!
//! <details>
//! <summary>
//!     <strong><code>single-triangle.ascii.ply</code>:</strong>
//! </summary>
//! </details>
//!
//! ```plaintext
//! ply
//! format ascii 1.0
//! element vertex 3
//! property float x
//! property float y
//! end_header
//! 0.0 1.0
//! 0.0 0
//! .0 -1.0
//! ```
//!
//! ----
//!
//! <details>
//! <summary>
//!     <strong><code>single-triangle.binary-le.ply</code>:</strong>
//! </summary>
//! </details>
//!
//! ```plaintext
//! ply
//! format binary_little_endian 1.0
//! element vertex 3
//! property float x
//! property float y
//! end_header
//! \x00\x00\x00\x00\x00\x00\x80\x3f
//! \x00\x00\x00\x00\x00\x00\x00\x00
//! \x00\x00\x00\x00\x00\x00\x80\xbf
//! ```
#![doc = include_str!("SUPPLEMENT.md")]
#![doc = include_str!("SYNTAX.md")]
#![doc = include_str!("LICENSE.md")]

pub mod header;
pub mod object;
pub mod payload;

pub use crate::{
    error::Error,
    function::{Decoder, DecoderWith, Encoder},
};
pub use header::Header;
pub use object::Object;
pub use payload::Payload;
