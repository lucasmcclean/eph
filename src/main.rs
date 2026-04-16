mod app;
mod commands;
mod storage;
mod sync;
mod task;

use clap::{Parser, Subcommand};

pub const BIN_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command
            .clone()
            .unwrap_or_else(|| Command::Interact(commands::Interact::default()))
    }
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    #[command(visible_alias = "i")]
    Interact(commands::Interact),

    #[command(visible_alias = "a")]
    Add(commands::Add),

    #[command(visible_alias = "d")]
    Done(commands::Done),

    #[command(visible_alias = "e")]
    Edit(commands::Edit),

    #[command(visible_alias = "l")]
    List(commands::List),

    #[command(visible_alias = "s")]
    Sync(commands::Sync),
}

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Command::Interact(interact) => interact.run(),
        Command::Add(add) => add.run(),
        Command::Done(done) => done.run(),
        Command::Edit(edit) => edit.run(),
        Command::List(list) => list.run(),
        Command::Sync(sync) => sync.run(),
    }
}
