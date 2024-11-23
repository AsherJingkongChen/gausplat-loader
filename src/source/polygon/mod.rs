//! The module `polygon` can read and write polygon files (`*.ply`).
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
#![doc = include_str!("SYNTAX.md")]
#![doc = include_str!("LICENSE.md")]

pub mod header;

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};
pub use header::*;
