use std::path::{Path, PathBuf};

use crate::BIN_NAME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepoPath {
    Default(PathBuf),
    // Custom(PathBuf),
}

impl Default for RepoPath {
    fn default() -> Self {
        let dirs = directories::ProjectDirs::from("", "", BIN_NAME)
            .expect("Couldn't determine repo directory.");

        Self::Default(dirs.data_dir().join("repo"))
    }
}

impl AsRef<Path> for RepoPath {
    fn as_ref(&self) -> &Path {
        match self {
            RepoPath::Default(path) => path.as_path(),
            // DataPath::Custom(path) => path.as_path(),
        }
    }
}
