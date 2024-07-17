use super::ImageFile;

pub type ImageFiles<R> = dashmap::DashMap<String, ImageFile<R>>;
