pub use super::*;

use crate::function::{read_byte_after, read_bytes_before};
use std::{
    collections::HashMap,
    fmt,
    io::Read,
    sync::{LazyLock, RwLock},
};

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
    RwLock<HashMap<Box<[u8]>, ScalarProperty>>,
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
/// <scalar-property> :=
///     | [{" "}] <kind> " "
///
/// <kind> :=
///     | "float" | "int" | "uchar"
///     | "float32" | "int32" | "uint8"
///     | ...
///     | <ascii-string>
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScalarProperty {
    pub kind: AsciiString,
    pub size: u64,
}

impl ScalarProperty {
    #[inline]
    pub fn try_new<K: AsRef<[u8]>>(
        kind: K,
        size: u64,
    ) -> Result<Self, Error> {
        let kind = kind.as_ref().into_ascii_string().map_err(|err| {
            Error::InvalidAscii(
                String::from_utf8_lossy(&err.into_source()).into_owned(),
            )
        })?;
        Ok(Self { kind, size })
    }

    pub fn register<K: AsRef<[u8]>>(
        kind: K,
        size: u64,
    ) -> Result<Option<ScalarProperty>, Error> {
        let kind = kind.as_ref();

        #[cfg(debug_assertions)]
        log::info!(
            target: "polygon::property::scalar",
            "register ({})",
            String::from_utf8_lossy(kind),
        );

        let property = ScalarProperty::try_new(kind, size)?;

        Ok(SCALAR_PROPERTY_DOMAIN
            .write()
            .expect("Poisoned")
            .insert(kind.into(), property))
    }

    pub fn search<K: AsRef<[u8]>>(kind: K) -> Option<ScalarProperty> {
        SCALAR_PROPERTY_DOMAIN
            .read()
            .expect("Poisoned")
            .get(kind.as_ref())
            .cloned()
    }

    pub fn unregister<K: AsRef<[u8]>>(kind: K) -> Option<ScalarProperty> {
        let kind = kind.as_ref();

        #[cfg(debug_assertions)]
        log::info!(
            target: "polygon::property::scalar",
            "unregister ({})",
            String::from_utf8_lossy(kind),
        );

        // Keep the "list" property in the domain.
        if kind == LIST.kind.as_bytes() {
            return Some(LIST.to_owned());
        }

        SCALAR_PROPERTY_DOMAIN
            .write()
            .expect("Poisoned")
            .remove(kind)
    }
}

impl Decoder for ScalarProperty {
    type Err = Error;

    fn decode(reader: &mut impl Read) -> Result<Self, Self::Err> {
        let mut kind = vec![read_byte_after(reader, |b| b == b' ')?
            .ok_or_else(|| Error::MissingToken("<kind>".into()))?];
        kind.extend(read_bytes_before(reader, |b| b == b' ', 8)?);

        Self::search(kind.as_slice()).ok_or_else(|| {
            Error::UnknownPolygonPropertyKind(
                String::from_utf8_lossy(&kind).into_owned(),
            )
        })
    }
}

impl Default for ScalarProperty {
    #[inline]
    fn default() -> Self {
        FLOAT.to_owned()
    }
}

impl fmt::Display for ScalarProperty {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

macro_rules! define_scalar_property {
    ($kind:ident, $size:literal) => {
        paste::paste! {
            pub static [<$kind:upper>]: std::sync::LazyLock<ScalarProperty> =
                std::sync::LazyLock::new(|| {
                    let kind = stringify!([<$kind:lower>]);
                    ScalarProperty::try_new(kind, $size)
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
        let output = ScalarProperty::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"float64 ");
        let target = FLOAT64.to_owned();
        let output = ScalarProperty::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"  int ");
        let target = INT.to_owned();
        let output = ScalarProperty::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"uchar ");
        let target = UCHAR.to_owned();
        let output = ScalarProperty::decode(source).unwrap();
        assert_eq!(output, target);

        let source = &mut Cursor::new(b"uchar\n");
        ScalarProperty::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"example ");
        ScalarProperty::decode(source).unwrap_err();

        let source = &mut Cursor::new(b"");
        ScalarProperty::decode(source).unwrap_err();
    }

    #[test]
    fn default() {
        use super::*;

        ScalarProperty::search(ScalarProperty::default().kind).unwrap();
    }

    #[test]
    fn display() {
        use super::*;

        let target = "float";
        let output = format!("{}", *FLOAT);
        assert_eq!(output, target);

        let target = "float32";
        let output = format!("{}", *FLOAT32);
        assert_eq!(output, target);
    }

    #[test]
    fn register_and_search_and_unregister() {
        use super::*;

        ScalarProperty::search("uint").unwrap();

        let target = UINT.to_owned();
        let output = ScalarProperty::register("uint", 4).unwrap().unwrap();
        assert_eq!(output, target);

        let output = ScalarProperty::search("uint").unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = ScalarProperty::search("example");
        assert_eq!(output, target);

        let target = None;
        let output = ScalarProperty::register("example", 1).unwrap();
        assert_eq!(output, target);

        let target = ScalarProperty::try_new("example", 1).unwrap();
        let output = ScalarProperty::search("example").unwrap();
        assert_eq!(output, target);

        let output = ScalarProperty::unregister("example").unwrap();
        assert_eq!(output, target);

        let target = None;
        let output = ScalarProperty::search("example");
        assert_eq!(output, target);
    }

    #[test]
    fn search_on_invalid_ascii_kind() {
        use super::*;

        let target = None;
        let output = ScalarProperty::search("\u{ae}");
        assert_eq!(output, target);
    }

    #[test]
    fn try_new_on_invalid_ascii_kind() {
        use super::*;

        ScalarProperty::try_new("\u{ae}", 1).unwrap_err();
    }

    #[test]
    fn unregister_on_invalid_ascii_kind() {
        use super::*;

        let target = None;
        let output = ScalarProperty::unregister("\u{ae}");
        assert_eq!(output, target);
    }

    #[test]
    fn unregister_list_no_effect() {
        use super::*;

        let target = LIST.to_owned();
        let output = ScalarProperty::unregister(&LIST.kind).unwrap();
        assert_eq!(output, target);

        let output = ScalarProperty::search(&LIST.kind).unwrap();
        assert_eq!(output, target);
    }
}
