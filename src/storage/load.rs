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

    let tasks = task_file
        .tasks
        .into_iter()
        .map(|(id, data)| Task::from_data(id, data))
        .collect();

    Ok(tasks)
}
