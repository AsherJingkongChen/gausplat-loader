pub use super::File;
pub use crate::{error::Error, function::Opener};

use std::{fs, path};

pub type Files<R> = std::collections::HashMap<String, File<R>>;

impl Opener for Files<fs::File> {
    fn open(path: impl AsRef<path::Path>) -> Result<Self, Error> {
        let files = fs::read_dir(path)?
            .map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                let file = File::open(path)?;

                Ok((file.name.to_owned(), file))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat_importer::scene", "colmap::Files::open");

        files
    }
}
