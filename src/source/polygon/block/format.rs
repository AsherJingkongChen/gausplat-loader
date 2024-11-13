pub struct FormatBlock {
    pub version: String,
    pub variant: FormatBlockVariant,
}

impl FormatBlock {
    #[inline]
    pub const fn kind(&self) -> &str {
        match self.variant {
            FormatBlockVariant::Ascii => "ascii",
            FormatBlockVariant::BinaryBigEndian => "binary_big_endian",
            FormatBlockVariant::BinaryLittleEndian => "binary_little_endian",
        }
    }
}

pub enum FormatBlockVariant {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}
