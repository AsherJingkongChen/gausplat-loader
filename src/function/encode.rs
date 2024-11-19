pub use crate::error::Error;

use std::io::Write;

/// Newline, that is, CRLF on Windows targets.
#[cfg(windows)]
pub const NEWLINE: &[u8; 2] = b"\r\n";
/// Newline, that is, LF on non-Windows targets.
#[cfg(not(windows))]
pub const NEWLINE: &[u8; 1] = b"\n";

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
