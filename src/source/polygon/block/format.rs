pub struct FormatBlock {
    pub version: String,
    pub variant: FormatBlockVariant,
}

pub enum FormatBlockVariant {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl FormatBlock {
    #[inline]
    pub const fn kind(&self) -> &str {
        match self.variant {
            FormatBlockVariant::BinaryLittleEndian => "binary_little_endian",
            FormatBlockVariant::Ascii => "ascii",
            FormatBlockVariant::BinaryBigEndian => "binary_big_endian",
        }
    }
}
