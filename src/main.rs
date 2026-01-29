mod add;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "eph")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    /// Returns the command; defaults to [`Command::Interactive`] if one is not
    /// provided.
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Interactive)
    }
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    Interactive,
    Add(add::Command),
    Edit,
    Done,
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command() {
        Command::Interactive => println!("interactive"),
        Command::Add(a) => a.run(),
        Command::Edit => println!("edit"),
        Command::Done => println!("done"),
        Command::List => println!("list"),
    }
}
