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

pub trait Encoder
where
    Self: Sized,
{
    type Err;

    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err>;
}

#[inline]
pub fn string_from_vec_ascii(vec: Vec<u8>) -> Result<String, Error> {
    let string = string_from_vec(vec)?;
    if !string.is_ascii() {
        return Err(Error::InvalidAscii(string));
    }
    Ok(string)
}

#[inline]
pub fn string_from_vec(vec: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(vec)
        .map_err(|err| Error::InvalidUtf8(String::from_utf8_lossy(err.as_bytes()).into()))
}
