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

        let mut writer = std::io::Cursor::new(Vec::new());

        write_any(&mut writer, &[0x00000201, 0x00500004]).unwrap();
        let output = writer.into_inner();
        let target = vec![0x01, 0x02, 0x00, 0x00, 0x04, 0x00, 0x50, 0x00];
        assert_eq!(output, target);
    }
}
