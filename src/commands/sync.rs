use clap::Parser;

use crate::app::tasks::{SyncStatus, sync_tasks};

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Sync {}

impl Sync {
    pub fn run(self) {
        match sync_tasks() {
            SyncStatus::Synced => println!("Successfully synced"),
            SyncStatus::Failed { msg } => println!("Sync failed: {}", msg),
        }
    }
}
