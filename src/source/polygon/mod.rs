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

use crate::function::{
    decode::{
        is_space, read_byte_after, read_bytes, read_bytes_before,
        read_bytes_before_newline, read_bytes_const, take_newline,
    },
    encode::{NEWLINE, SPACE},
};
use std::io::{Read, Write};

// TODO: Syntax documentation
