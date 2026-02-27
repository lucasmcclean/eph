use clap::Parser;

use crate::{
    storage,
    task::{Priority, Task},
};

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Add {
    title: String,

    #[arg(short, long, default_value = "default")]
    context: String,

    #[arg(
        short,
        long,
        default_value_t = Priority::Low,
        help = "Priority: 1–4 (high, medium, low, backlog)"
    )]
    priority: Priority,

    #[arg(short, long, value_delimiter = ',')]
    tags: Vec<String>,

    #[arg(short, long)]
    description: Option<String>,
}

impl Add {
    pub fn run(self) {
        let task = Task::new(self.title, self.context, self.priority)
            .with_tags(self.tags)
            .with_description_opt(self.description);
        let result = storage::append(storage::DataPath::default(), task);
        match result {
            Ok(_) => println!("Task successfully added"),
            Err(err) => println!("{}", err),
        }
    }
}
