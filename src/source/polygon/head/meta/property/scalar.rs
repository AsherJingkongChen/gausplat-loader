pub use super::*;

use std::{
    collections::HashMap,
    fmt,
    sync::{LazyLock, RwLock},
};

// NOTE: "list" is a specialized scalar property.
define_scalar_property!(LIST, 0);
define_scalar_property!(CHAR, 1);
define_scalar_property!(INT8, 1);
define_scalar_property!(UCHAR, 1);
define_scalar_property!(UINT8, 1);
define_scalar_property!(FLOAT16, 2);
define_scalar_property!(HALF, 2);
define_scalar_property!(INT16, 2);
define_scalar_property!(SHORT, 2);
define_scalar_property!(UINT16, 2);
define_scalar_property!(USHORT, 2);
define_scalar_property!(FLOAT, 4);
define_scalar_property!(FLOAT32, 4);
define_scalar_property!(INT, 4);
define_scalar_property!(INT32, 4);
define_scalar_property!(UINT, 4);
define_scalar_property!(UINT32, 4);
define_scalar_property!(DOUBLE, 8);
define_scalar_property!(FLOAT64, 8);
define_scalar_property!(INT64, 8);
define_scalar_property!(LONG, 8);
define_scalar_property!(UINT64, 8);
define_scalar_property!(ULONG, 8);

/// A hash map whose key is the kind of scalar property,
/// which is guaranteed to be an ASCII string.
static SCALAR_PROPERTY_DOMAIN: LazyLock<
    RwLock<HashMap<Box<[u8]>, ScalarPropertyMeta>>,
> = LazyLock::new(|| {
    [
        &LIST, &CHAR, &INT8, &UCHAR, &UINT8, &FLOAT16, &HALF, &INT16, &SHORT,
        &UINT16, &USHORT, &FLOAT, &FLOAT32, &INT, &INT32, &UINT, &UINT32,
        &DOUBLE, &FLOAT64, &INT64, &LONG, &UINT64, &ULONG,
    ]
    .into_iter()
    .map(|p| (p.kind.as_bytes().into(), (*p).to_owned()))
    .collect::<HashMap<_, _>>()
    .into()
});

/// ## Syntax
///
/// ```plaintext
/// <scalar-property-meta> :=
///     | [{" "}] <kind> " "
///
/// <kind> :=
///     | "float" | "int" | "uchar"
///     | "float32" | "int32" | "uint8"
///     | ...
///     | <ascii-string>
/// ```
///
/// ### Syntax Reference
///
/// - [`AsciiString`]
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarPropertyMeta {
    pub kind: AsciiString,
    pub size: usize,
}

impl ScalarPropertyMeta {
    #[inline]
    pub fn new<K: AsRef<[u8]>>(
        kind: K,
        size: usize,
    ) -> Result<Self, Error> {
        let kind = kind.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(err.into_source()).into_owned(),
            )
        })?;
        Ok(Self { kind, size })
    }

    #[inline]
    pub fn register<K: AsRef<[u8]>>(
        kind: K,
        size: usize,
    ) -> Result<Option<ScalarPropertyMeta>, Error> {
        let kind = kind.as_ref();

        #[cfg(all(debug_assertions, not(test)))]
        log::info!(
            target: "polygon::property::scalar",
            "register ({})",
            String::from_utf8_lossy(kind),
        );

        // Retaining the special properties.
        // Forbidding zero-sized properties.
        if kind == b"list" || size == 0 {
            Err(Error::InvalidPolygonPropertyKind(
                String::from_utf8_lossy(kind).into_owned(),
            ))?;
        }

        Ok(SCALAR_PROPERTY_DOMAIN
            .write()
            .expect("Poisoned")
            .insert(kind.into(), ScalarPropertyMeta::new(kind, size)?))
    }

    #[inline]
    pub fn search<K: AsRef<[u8]>>(kind: K) -> Option<ScalarPropertyMeta> {
        SCALAR_PROPERTY_DOMAIN
            .read()
            .expect("Poisoned")
            .get(kind.as_ref())
            .cloned()
    }

    #[inline]
    pub fn unregister<K: AsRef<[u8]>>(kind: K) -> Option<ScalarPropertyMeta> {
        let kind = kind.as_ref();

        #[cfg(all(debug_assertions, not(test)))]
        log::info!(
            target: "polygon::property::scalar",
            "unregister ({})",
            String::from_utf8_lossy(kind),
        );

        // Retaining the special properties.
        if kind == b"list" {
            None?;
        }

        SCALAR_PROPERTY_DOMAIN
            .write()
            .expect("Poisoned")
            .remove(kind)
    }
}

impl Decoder for ScalarPropertyMeta {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut kind = vec![read_byte_after(reader, is_space)?
            .ok_or_else(|| Error::MissingToken("<kind>".into()))?];
        kind.extend(read_bytes_before(reader, is_space, 8)?);

        Self::search(kind.as_slice()).ok_or_else(|| {
            Error::InvalidPolygonPropertyKind(
                String::from_utf8_lossy(&kind).into_owned(),
            )
        })
    }
}

impl Default for ScalarPropertyMeta {
    #[inline]
    fn default() -> Self {
        FLOAT.to_owned()
    }
}

impl fmt::Debug for ScalarPropertyMeta {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "\"{}\"", self.kind)
    }
}

impl Encoder for ScalarPropertyMeta {
    type Err = Error;

    #[inline]
    fn encode(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), Self::Err> {
        writer.write_all(self.kind.as_bytes())?;
        Ok(writer.write_all(SPACE)?)
    }
}

macro_rules! define_scalar_property {
    ($kind:ident, $size:literal) => {
        paste::paste! {
            pub static [<$kind:upper>]: std::sync::LazyLock<ScalarPropertyMeta> =
                std::sync::LazyLock::new(|| {
                    let kind = stringify!([<$kind:lower>]);
                    ScalarPropertyMeta::new(kind, $size)
                        .expect(&format!("Invalid scalar property: {kind:?}"))
                });
        }
    };
}
use define_scalar_property;

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use super::*;
        use std::io::Cursor;

        let source = &mut Cursor::new(b"float ");
        let target = FLOAT.to_owned();
        let output = ScalarPropertyMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"float64 ");
        let target = FLOAT64.to_owned();
        let output = ScalarPropertyMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"  int ");
        let target = INT.to_owned();
        let output = ScalarPropertyMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"uchar ");
        let target = UCHAR.to_owned();
        let output = ScalarPropertyMeta::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"uchar\n");
        ScalarPropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"example ");
        ScalarPropertyMeta::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        ScalarPropertyMeta::decode(source).unwrap_err();
    }

    #[test]
    fn default() {
        use super::*;

        ScalarPropertyMeta::search(ScalarPropertyMeta::default().kind).unwrap();
    }

    #[test]
    fn register_and_search_and_unregister() {
        use super::*;

        ScalarPropertyMeta::search("uint").unwrap();

        let target = UINT.to_owned();
        let output = ScalarPropertyMeta::register("uint", 4).unwrap().unwrap();
        assert_eq!(output, target);

        let output = ScalarPropertyMeta::search("uint").unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = ScalarPropertyMeta::search("example");
        assert_eq!(output, target);

        let target = None;
        let output = ScalarPropertyMeta::register("example", 1).unwrap();
        assert_eq!(output, target);

        let target = ScalarPropertyMeta::new("example", 1).unwrap();
        let output = ScalarPropertyMeta::search("example").unwrap();
        assert_eq!(output, target);

        let output = ScalarPropertyMeta::unregister("example").unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = ScalarPropertyMeta::search("example");
        assert_eq!(output, target);
    }

    #[test]
    fn register_on_list() {
        use super::*;

        ScalarPropertyMeta::search("list").unwrap();
        ScalarPropertyMeta::register("list", 0).unwrap_err();
    }

    #[test]
    fn search_on_invalid_ascii_kind() {
        use super::*;

        let target = None;
        let output = ScalarPropertyMeta::search("\u{ae}");
        assert_eq!(output, target);
    }

    #[test]
    fn new_on_invalid_ascii_kind() {
        use super::*;

        ScalarPropertyMeta::new("\u{ae}", 1).unwrap_err();
    }

    #[test]
    fn unregister_on_invalid_ascii_kind() {
        use super::*;

        let target = None;
        let output = ScalarPropertyMeta::unregister("\u{ae}");
        assert_eq!(output, target);
    }

    #[test]
    fn unregister_on_list() {
        use super::*;

        let target = None;
        let output = ScalarPropertyMeta::unregister("list");
        assert_eq!(output, target);

        let target = LIST.to_owned();
        let output = ScalarPropertyMeta::search("list").unwrap();
        assert_eq!(output, target);
    }
}
