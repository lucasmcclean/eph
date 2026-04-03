use clap::Parser;
use uuid::Uuid;

use crate::app::{self, tasks::CompletionStatus};

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Done {
    id: Uuid,
}

impl Done {
    pub fn run(&self) {
        match app::complete_task(self.id) {
            CompletionStatus::Marked => println!("Marked complete"),
            CompletionStatus::AlreadyMarked => println!("Already marked complete"),
            CompletionStatus::NotFound => println!("No task with that ID exists"),
            CompletionStatus::StorageError(err) => {
                eprintln!("Failed to save task: {}", err)
            }
        }
    }
}
