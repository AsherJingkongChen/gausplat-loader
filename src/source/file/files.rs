pub use super::File;
pub use crate::{error::Error, function::Opener};

use std::{
    fs,
    path::{Path, PathBuf},
};

pub type Files<S> = crate::collection::IndexMap<PathBuf, File<S>>;

impl Opener for Files<fs::File> {
    fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let files = fs::read_dir(path)?
            .map(|entry| {
                let path = entry?.path();
                let file = File::open(&path)?;

                Ok((path, file))
            })
            .collect();

        #[cfg(debug_assertions)]
        log::debug!(target: "gausplat-loader::source::file", "Files::open");

        files
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open() {
        use super::*;

        let source = "examples/data/hello-world/";
        let files = Files::open(source).unwrap();

        let target = false;
        let output = files.is_empty();
        assert_eq!(output, target);
    }
}
