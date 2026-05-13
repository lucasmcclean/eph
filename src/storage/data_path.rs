use std::path::{Path, PathBuf};

use crate::BIN_NAME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataPath(PathBuf);

impl Default for DataPath {
    fn default() -> Self {
        let dirs = directories::ProjectDirs::from("", "", BIN_NAME)
            .expect("Couldn't determine data directory.");

        Self(dirs.data_dir().join("tasks.toml"))
    }
}

impl AsRef<Path> for DataPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
