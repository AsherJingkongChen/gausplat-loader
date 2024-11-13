pub struct PropertyBlock {
    pub id: u32,
    pub name: String,
    pub variant: PropertyBlockVariant,
}

pub enum PropertyBlockVariant {
    List(ListProperty),
    Scalar(ScalarProperty),
}

pub struct ListProperty {
    pub count: ScalarProperty,
    pub entry: ScalarProperty,
}

#[derive(Debug)]
pub struct ScalarProperty {
    // NOTE: It allows user-defined property kind
    pub kind: String,
    pub size: u32,
}

// NOTE: Pre-defined scalar properties
pub mod scalar {
    define_scalar_property!(char, 1);
    define_scalar_property!(int8, 1);
    define_scalar_property!(ichar, 1);
    define_scalar_property!(uchar, 1);
    define_scalar_property!(float16, 2);
    define_scalar_property!(half, 2);
    define_scalar_property!(int16, 2);
    define_scalar_property!(short, 2);
    define_scalar_property!(uint16, 2);
    define_scalar_property!(ushort, 2);
    define_scalar_property!(float, 4);
    define_scalar_property!(float32, 4);
    define_scalar_property!(int, 4);
    define_scalar_property!(int32, 4);
    define_scalar_property!(uint, 4);
    define_scalar_property!(uint32, 4);
    define_scalar_property!(double, 8);
    define_scalar_property!(float64, 8);
    define_scalar_property!(int64, 8);
    define_scalar_property!(long, 8);
    define_scalar_property!(uint64, 8);
    define_scalar_property!(ulong, 8);

    #[macro_export]
    macro_rules! define_scalar_property {
        ($kind:ident, $size:literal) => {
            paste::paste! {
                pub static [<$kind:upper>]: std::sync::LazyLock<super::ScalarProperty> =
                    std::sync::LazyLock::new(|| super::ScalarProperty {
                        kind: stringify!($kind).into(),
                        size: $size,
                    });
            }
        };
    }
    use define_scalar_property;
}
