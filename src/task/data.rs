use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::{Priority, Task};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskData {
    pub title: String,
    pub context: String,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<Task> for (Uuid, TaskData) {
    fn from(task: Task) -> (Uuid, TaskData) {
        (
            task.id(),
            TaskData {
                title: task.title,
                context: task.context,
                priority: task.priority,
                tags: task.tags,
                description: task.description,
                created_at: task.created_at,
                completed_at: task.completed_at,
            },
        )
    }
}
