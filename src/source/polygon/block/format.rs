pub struct FormatBlock {
    pub variant: FormatVariant,
    pub version: String,
}

pub enum FormatVariant {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl FormatBlock {
    #[inline]
    pub const fn kind(&self) -> &str {
        match self.variant {
            FormatVariant::BinaryLittleEndian => "binary_little_endian",
            FormatVariant::Ascii => "ascii",
            FormatVariant::BinaryBigEndian => "binary_big_endian",
        }
    }
}
