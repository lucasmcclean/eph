use clap::Parser;

use crate::storage::{DataPath, RepoPath, sync};

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Sync {}

impl Sync {
    pub fn run(self) {
        if let Err(err) = sync(RepoPath::default(), DataPath::default()) {
            println!("{}", err);
        }
    }
}
