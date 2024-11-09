pub use crate::error::Error;
pub use bytemuck::Pod;

use std::io::Write;

pub trait Encoder
where
    Self: Sized,
{
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Error>;
}

pub fn write_any<T: Pod>(
    writer: &mut impl Write,
    value: &T,
) -> Result<(), Error> {
    Ok(writer.write_all(bytemuck::bytes_of(value))?)
}

pub fn write_string_with_zero(
    writer: &mut impl Write,
    value: &str,
) -> Result<(), Error> {
    writer.write_all(value.as_bytes())?;
    Ok(writer.write_all(&[0])?)
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
    fn write_string_with_zero() {
        use super::*;

        let source = "Hello, World!";
        let mut writer = std::io::Cursor::new(Vec::new());

        let target =
            &include_bytes!("../../examples/data/hello-world/ascii+binary.dat")
                [8..8 + 13 + 1];
        write_string_with_zero(&mut writer, source).unwrap();
        let output = writer.into_inner();
        assert_eq!(output, target);
    }
}
