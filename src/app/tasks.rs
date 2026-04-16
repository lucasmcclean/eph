use std::error::Error;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    storage::{self, sync},
    task::{Task, TaskFilter, TaskPatch},
};

fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    storage::load(storage::DataPath::default())
}

fn store_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn Error>> {
    storage::store(storage::DataPath::default(), &tasks)
}

fn append_task(task: Task) -> Result<(), Box<dyn Error>> {
    storage::append(storage::DataPath::default(), task)
}

pub fn add_task(task: Task) -> Result<(), Box<dyn Error>> {
    append_task(task)
}

pub fn filter_tasks(filter: &TaskFilter) -> Vec<Task> {
    let tasks = load_tasks().unwrap();

    tasks
        .into_iter()
        .filter(|task| filter.matches(task))
        .collect()
}

pub enum CompletionStatus {
    Marked,
    AlreadyMarked,
    NotFound,
    StorageError(Box<dyn Error>),
}

pub fn complete_task(identifier: Uuid) -> CompletionStatus {
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => return CompletionStatus::StorageError(e),
    };

    let Some(task) = tasks.iter_mut().find(|task| task.id() == identifier) else {
        return CompletionStatus::NotFound;
    };

    if task.completed_at.is_some() {
        return CompletionStatus::AlreadyMarked;
    }

    task.completed_at = Some(Utc::now());

    match store_tasks(tasks) {
        Ok(_) => CompletionStatus::Marked,
        Err(e) => CompletionStatus::StorageError(e),
    }
}

pub enum EditStatus {
    Updated,
    NotFound,
    StorageError(Box<dyn Error>),
}

pub fn edit_task(task_patch: TaskPatch) -> EditStatus {
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => return EditStatus::StorageError(e),
    };

    let Some(task) = tasks.iter_mut().find(|task| task.id() == task_patch.id()) else {
        return EditStatus::NotFound;
    };

    task_patch.apply_to(task);

    match store_tasks(tasks) {
        Ok(_) => EditStatus::Updated,
        Err(e) => EditStatus::StorageError(e),
    }
}

pub enum SyncStatus {
    Synced,
    Failed { msg: String },
}

pub fn sync_tasks() -> SyncStatus {
    if let Err(err) = sync(storage::RepoPath::default(), storage::DataPath::default()) {
        return SyncStatus::Failed {
            msg: err.to_string(),
        };
    }
    SyncStatus::Synced
}
