use super::ImageFile;
use dashmap::DashMap;

pub type ImageFiles<R> = DashMap<String, ImageFile<R>>;
