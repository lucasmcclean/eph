use std::path::{Path, PathBuf};

use crate::BIN_NAME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepoPath(PathBuf);

impl Default for RepoPath {
    fn default() -> Self {
        let dirs = directories::ProjectDirs::from("", "", BIN_NAME)
            .expect("Couldn't determine repo directory.");

        Self(dirs.data_dir().join("repo"))
    }
}

impl AsRef<Path> for RepoPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
