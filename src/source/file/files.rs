pub use super::File;
pub use crate::{error::Error, function::Opener};

use std::{
    fs,
    path::{Path, PathBuf},
};

pub type Files<S> = crate::collection::IndexMap<PathBuf, File<S>>;

impl Opener for Files<fs::File> {
    /// Opening all files matching the glob pattern.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gausplat_loader::source::file::{Files, Opener};
    ///
    /// let files = Files::open("examples/data/hello-world/*").unwrap();
    /// assert!(!files.is_empty());
    /// ```
    fn open(pattern: impl AsRef<Path>) -> Result<Self, Error> {
        let pattern = pattern.as_ref();
        let matcher =
            globset::GlobBuilder::new(pattern.to_str().ok_or_else(|| {
                Error::InvalidUtf8(pattern.as_os_str().to_owned())
            })?)
            .literal_separator(true)
            .build()?
            .compile_matcher();
        let rootdir = pattern
            .ancestors()
            .find(|path| path.is_dir())
            .unwrap_or(&Path::new("."));
        let files = walkdir::WalkDir::new(rootdir)
            .contents_first(true)
            .follow_links(true)
            .into_iter()
            .filter(|entry| {
                entry.as_ref().is_ok_and(|entry| {
                    entry.file_type().is_file()
                        && matcher.is_match(entry.path())
                })
            })
            .map(|entry| {
                let path = entry.as_ref().expect("Unreachable").path();
                Ok((path.to_owned(), File::open(path)?))
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

        let target = true;
        let output = files.is_empty();
        assert_eq!(output, target);

        let source = "examples/data/hello-world/*";
        let files = Files::open(source).unwrap();

        let target = false;
        let output = files.is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn open_on_symlink() {
        use super::*;

        let source = "examples/data/hello-world.symlink";
        let files = Files::open(source).unwrap();

        let target = true;
        let output = files.is_empty();
        assert_eq!(output, target);

        let source = "examples/data/hello-world.symlink/*";
        let files = Files::open(source).unwrap();

        let target = false;
        let output = files.is_empty();
        assert_eq!(output, target);

        let source = "examples/data/hello-world/ascii.symlink.txt";
        let files = Files::open(source).unwrap();

        let target = false;
        let output = files.is_empty();
        assert_eq!(output, target);

        let source = "examples/data/hello-world.symlink/*.symlink.txt";
        let files = Files::open(source).unwrap();

        let target = false;
        let output = files.is_empty();
        assert_eq!(output, target);
    }

    #[test]
    fn open_on_invalid_utf8() {
        use super::*;

        // SAFETY: The sequence is deliberately invalid UTF-8.
        let source = unsafe {
            std::ffi::OsStr::from_encoded_bytes_unchecked(
                b"examples/data/hello-world/\x8e\xcd*",
            )
        };
        Files::open(source).unwrap_err();
    }
}
