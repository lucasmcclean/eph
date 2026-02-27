use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFile {
    pub tasks: Vec<Task>,
}
