mod commands;
mod storage;
mod task;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Interact)
    }
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    #[command(visible_alias = "i")]
    Interact,

    #[command(visible_alias = "a")]
    Add(commands::Add),

    #[command(visible_alias = "d")]
    Done(commands::Done),

    #[command(visible_alias = "e")]
    Edit(commands::Edit),

    #[command(visible_alias = "l")]
    List(commands::List),
}

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Command::Interact => println!("interact"),
        Command::Add(a) => a.run(),
        Command::Done(d) => d.run(),
        Command::Edit(e) => e.run(),
        Command::List(l) => l.run(),
    }
}
