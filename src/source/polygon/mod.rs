//! The module `polygon` can read and write polygon files (PLY).
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

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::decode::{read_bytes_before_many_const, read_newline};
use std::io::Read;

pub fn read_polygon_header(reader: &mut impl Read) -> Result<String, Error> {
    let mut header = read_bytes_before_many_const(reader, b"end_header", 1024)?;
    header.extend(b"end_header");
    header.extend(read_newline(reader)?);
    Ok("".to_string())
}
