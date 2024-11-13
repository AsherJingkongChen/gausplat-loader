pub struct DataBlock {
    pub id: u32,
    pub variant: DataBlockVariant,
}

pub enum DataBlockVariant {
    Packed(DataPacked),
    Planar(DataPlanar),
}

pub struct DataPacked {
    pub inner: Vec<u8>,
}

pub struct DataPlanar {
    pub inner: Vec<Vec<u8>>,
}
