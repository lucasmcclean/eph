use std::error::Error;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    storage,
    task::{Task, TaskPatch},
};

pub fn add_task(task: Task) -> Result<(), Box<dyn Error>> {
    storage::append(storage::DataPath::default(), task)
}

pub enum CompletionStatus {
    Marked,
    AlreadyMarked,
    NotFound,
    StorageError(Box<dyn Error>),
}

pub fn complete_task(identifier: Uuid) -> CompletionStatus {
    let mut tasks = match storage::load(storage::DataPath::default()) {
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

    match storage::store(storage::DataPath::default(), &tasks) {
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
    let mut tasks = match storage::load(storage::DataPath::default()) {
        Ok(tasks) => tasks,
        Err(e) => return EditStatus::StorageError(e),
    };

    let Some(task) = tasks.iter_mut().find(|task| task.id() == task_patch.id()) else {
        return EditStatus::NotFound;
    };

    task_patch.apply_to(task);

    match storage::store(storage::DataPath::default(), &tasks) {
        Ok(_) => EditStatus::Updated,
        Err(e) => EditStatus::StorageError(e),
    }
}
