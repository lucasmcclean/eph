use std::error::Error;

use crate::{storage, task::Task};

pub fn add_task(task: Task) -> Result<(), Box<dyn Error>> {
    storage::append(storage::DataPath::default(), task)
}
