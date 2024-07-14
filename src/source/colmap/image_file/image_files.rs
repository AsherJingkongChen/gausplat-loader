use super::ImageFile;
use std::collections::HashMap;

pub type ImageFiles<R> = HashMap<String, ImageFile<R>>;
