use std::path::PathBuf;

use directories::ProjectDirs;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

pub enum DataPath {
    Default,
    // Custom(PathBuf),
}

impl DataPath {
    pub fn resolve(self) -> PathBuf {
        match self {
            DataPath::Default => self.default_data_path(),
            // DataPath::Custom(path) => path,
        }
    }

    fn default_data_path(self) -> PathBuf {
        let dirs = ProjectDirs::from("", "", BIN_NAME).expect("Couldn't determine data directory.");
        dirs.data_dir().join("tasks.toml")
    }
}
