pub use crate::error::Error;
pub use bytemuck::Pod;

use std::io::Write;

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

/// Writing any type of data.
#[inline]
pub fn write_any<T: Pod>(
    writer: &mut impl Write,
    value: &T,
) -> Result<(), Error> {
    Ok(writer.write_all(bytemuck::bytes_of(value))?)
}

/// Writing all bytes.
#[inline]
pub fn write_bytes(
    writer: &mut impl Write,
    bytes: &[u8],
) -> Result<(), Error> {
    Ok(writer.write_all(bytes)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_any() {
        use super::*;

        let source = &[20241109_u32, 131452000];
        let mut writer = std::io::Cursor::new(Vec::new());

        let target =
            &include_bytes!("../../examples/data/hello-world/ascii+binary.dat")
                [..8];
        write_any(&mut writer, source).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }

    #[test]
    fn write_bytes() {
        use super::*;

        let source = b"Hello, World!";
        let mut writer = std::io::Cursor::new(Vec::new());

        let target =
            include_bytes!("../../examples/data/hello-world/ascii.txt");
        write_bytes(&mut writer, source).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }
}
