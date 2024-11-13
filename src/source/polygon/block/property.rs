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

pub struct ScalarProperty {
    pub kind: String,
    pub size: u32,
}
