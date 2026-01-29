mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "eph")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    // Returns the command; defaults to [`Command::Interactive`] if one is not
    // provided.
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Interactive)
    }
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    #[command(visible_alias = "i")]
    Interactive,

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
        Command::Interactive => println!("interactive"),
        Command::Add(a) => a.run(),
        Command::Done(d) => d.run(),
        Command::Edit(e) => e.run(),
        Command::List(l) => l.run(),
    }
}
