use crate::storage::file::TaskFile;
use crate::task::Task;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn store<P: AsRef<Path>>(path: P, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let task_file = TaskFile {
        tasks: tasks.to_vec(),
    };

    let tasks_toml = toml::to_string_pretty(&task_file)?;

    let mut tmp_file = NamedTempFile::new_in(path.parent().unwrap())?;
    tmp_file.write_all(tasks_toml.as_bytes())?;
    tmp_file.as_file_mut().sync_all()?;
    tmp_file.persist(path)?;

    if let Some(parent) = path.parent() {
        fs::File::open(parent)?.sync_all()?;
    }

    Ok(())
}

pub fn append<P: AsRef<Path>>(path: P, task: Task) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();

    let mut tasks = super::load(path)?;
    tasks.push(task);

    super::store(path, &tasks)?;

    Ok(())
}
