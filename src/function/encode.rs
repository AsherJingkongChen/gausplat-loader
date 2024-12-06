//! Functions for encoding process.

pub use crate::error::Error;

use std::io::Write;

/// Newline, that is, CRLF on Windows targets.
#[cfg(windows)]
pub const NEWLINE: &[u8; 2] = b"\r\n";
/// Newline, that is, LF on non-Windows targets.
#[cfg(not(windows))]
pub const NEWLINE: &[u8; 1] = b"\n";
/// Null.
pub const NULL: &[u8; 1] = b"\0";
/// Space.
pub const SPACE: &[u8; 1] = b" ";

/// Encoding function.
pub trait Encoder: Sized {
    /// Error type.
    type Err;

    /// Encode bytes to the writer.
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err>;
}

/// Convert bytes to an ASCII string.
///
/// # Errors
///
/// It returns an error if the bytes are not a valid ASCII string.
#[inline]
pub fn string_from_vec_ascii(vec: Vec<u8>) -> Result<String, Error> {
    let string = string_from_vec(vec)?;
    if !string.is_ascii() {
        return Err(Error::InvalidAscii(string));
    }
    Ok(string)
}

/// Convert bytes to a UTF-8 string.
///
/// # Errors
///
/// It returns an error if the bytes are not a valid UTF-8 string.
#[inline]
pub fn string_from_vec(vec: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(vec)
        .map_err(|err| Error::InvalidUtf8(String::from_utf8_lossy(err.as_bytes()).into()))
}
