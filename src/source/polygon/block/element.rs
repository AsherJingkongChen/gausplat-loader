pub use super::*;

/// ## Syntax
///
/// ```plaintext
/// <element-block> :=
///     | <name> <size> <newline>
///
/// <name> :=
///     | [{" "}] <ascii-string> " "
///
/// <size> :=
///     | [{" "}] <usize>
///
/// <newline> :=
///     | ["\r"] "\n"
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
/// - [`usize`]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElementBlock {
    pub name: AsciiString,
    pub size: usize,
}

impl ElementBlock {
    #[inline]
    pub fn new<N: AsRef<[u8]>>(
        name: N,
        size: usize,
    ) -> Result<Self, Error> {
        let name = name.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(err.into_source()).into_owned())
        })?;
        Ok(Self { name, size })
    }
}

impl Decoder for ElementBlock {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut name = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<name>".into()))?];
        name.extend(read_bytes_before(reader, is_space, 16)?);
        let name = name.into_ascii_string().map_err(|err| {
            Error::InvalidAscii(String::from_utf8_lossy(&err.into_source()).into_owned())
        })?;

        let mut size = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<size>".into()))?];
        size.extend(read_bytes_before_newline(reader, 16)?);
        let size = std::str::from_utf8(&size)
            .map_err(|_| Error::InvalidUtf8(String::from_utf8_lossy(&size).into_owned()))?
            .parse()?;

        Ok(Self { name, size })
    }
}

impl Encoder for ElementBlock {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(SPACE)?;

        writer.write_all(self.size.to_string().as_bytes())?;
        Ok(writer.write_all(NEWLINE)?)
    }
}

impl ops::Deref for ElementBlock {
    type Target = AsciiString;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl ops::DerefMut for ElementBlock {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.name
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn accessbility() {
        use super::*;

        let mut element = ElementBlock::new("vertex", 10).unwrap();
        assert_eq!(element.name, "vertex");
        assert_eq!(*element, "vertex");

        *element = "vertex^2".into_ascii_string().unwrap();
        element.size = 10 * 10;
        assert_eq!(*element, "vertex^2");
    }

    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"color 7\n");
        let target = "color";
        let output = ElementBlock::decode(source).unwrap();
        assert_eq!(output.name, target);
        let target = 7;
        assert_eq!(output.size, target);

        let source = &mut Cursor::new(b"rgb 888 100\n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"     point    ");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"vertex 3 \n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b" ");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_ascii_name() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new("\u{ae} 3\n".as_bytes());
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn decode_on_invalid_usize_size() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"none -1\n");
        ElementBlock::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"unicode \x8e\xcd\n");
        ElementBlock::decode(source).unwrap_err();
    }

    #[test]
    fn new_on_invalid_ascii_name() {
        use super::*;

        ElementBlock::new("\u{ae}", 0).unwrap_err();
        ElementBlock::new("\u{a7}", 1).unwrap_err();
    }
}
