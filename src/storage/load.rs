use std::{fs, io, path::Path};

use crate::{storage::file::TaskFile, task::Task};

pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = path.as_ref();

    let tasks_toml = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        Err(err) => return Err(err.into()),
    };

    let task_file: TaskFile = toml::from_str(&tasks_toml)?;
    Ok(task_file.tasks)
}
