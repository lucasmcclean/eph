use chrono::Utc;
use clap::Parser;
use uuid::Uuid;

use crate::storage;

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Done {
    id: Uuid,
}

impl Done {
    pub fn run(&self) {
        let mut tasks = storage::load(storage::DataPath::default()).unwrap();

        let completed_task = tasks.iter_mut().find(|task| task.id() == self.id);

        match completed_task {
            Some(task) => {
                if task.completed_at.is_some() {
                    println!("Already marked completed");
                } else {
                    task.completed_at = Some(Utc::now());
                    storage::store(storage::DataPath::default(), &tasks)
                        .expect("Failed to save tasks");
                    println!("Successfully marked completed");
                }
            }
            None => println!("Item not found"),
        }
    }
}
