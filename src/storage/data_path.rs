use std::path::{Path, PathBuf};

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataPath {
    Default(PathBuf),
    // Custom(PathBuf),
}

impl Default for DataPath {
    fn default() -> Self {
        let dirs = directories::ProjectDirs::from("", "", BIN_NAME)
            .expect("Couldn't determine data directory.");

        Self::Default(dirs.data_dir().join("tasks.toml"))
    }
}

impl AsRef<Path> for DataPath {
    fn as_ref(&self) -> &Path {
        match self {
            DataPath::Default(path) => path.as_path(),
            // DataPath::Custom(path) => path.as_path(),
        }
    }
}
