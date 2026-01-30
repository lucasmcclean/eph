use clap::Parser;

use crate::priority::Priority;

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
    pub fn run(&self) {
        println!("add");
    }
}
