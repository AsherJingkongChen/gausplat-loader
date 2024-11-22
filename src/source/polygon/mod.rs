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

pub use crate::{
    error::Error,
    function::{Decoder, Encoder},
};

use crate::function::{
    decode::{read_bytes_before_many_const, read_newline},
    read_bytes_until_many_const,
};
use std::io::Read;

/// Reading the bytes of the header of a polygon file.
#[inline]
pub fn read_bytes_of_polygon_header(reader: &mut impl Read) -> Result<Vec<u8>, Error> {
    const CAPACITY: usize = 1 << 10;
    const START: &[u8; 3] = b"ply";
    const END: &[u8; 10] = b"end_header";

    read_bytes_until_many_const(reader, START)?;
    let mut header = Vec::with_capacity(CAPACITY);
    header.extend(START);
    header.extend(read_newline(reader)?);
    header.extend(read_bytes_before_many_const(reader, END, CAPACITY)?);
    header.extend(END);
    header.extend(read_newline(reader)?);
    Ok(header)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_bytes_of_polygon_header() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"ply\nformat ascii 1.0\nend_header\n\n \n \n");
        let target = b"ply\nformat ascii 1.0\nend_header\n";
        let output = read_bytes_of_polygon_header(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"\n \n ply\nformat ascii 1.0\nend_header\n");
        let target = b"ply\nformat ascii 1.0\nend_header\n";
        let output = read_bytes_of_polygon_header(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"ply\r\nformat ascii 1.0\nend_header\n");
        let target = b"ply\r\nformat ascii 1.0\nend_header\n";
        let output = read_bytes_of_polygon_header(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"ply\nformat ascii 1.0\nend_header");
        read_bytes_of_polygon_header(source).unwrap_err();

        let source = &mut Cursor::new(b"ply\n");
        read_bytes_of_polygon_header(source).unwrap_err();

        let source = &mut Cursor::new(b"ply");
        read_bytes_of_polygon_header(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        read_bytes_of_polygon_header(source).unwrap_err();
    }
}
