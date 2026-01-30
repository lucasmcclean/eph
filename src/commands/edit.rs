use clap::Parser;

use crate::task::Priority;

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
#[command(
    group(
        clap::ArgGroup::new("changes")
            .required(true)
            .args([
                "title",
                "context",
                "priority",
                "add_tags",
                "delete_tags",
                "new_tags",
                "description",
            ])
    )
)]
pub struct Edit {
    identifier: String,

    #[arg(short, long)]
    in_context: Option<String>,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(short = 'c', long)]
    to_context: Option<String>,

    #[arg(short, long, help = "Priority: one of high, medium, low, backlog")]
    priority: Option<Priority>,

    #[arg(short, long, value_delimiter = ',', conflicts_with = "new_tags")]
    add_tags: Vec<String>,

    #[arg(short, long, value_delimiter = ',', conflicts_with = "new_tags")]
    delete_tags: Vec<String>,

    #[arg(short, long, value_delimiter = ',', conflicts_with_all = ["add_tags", "delete_tags"])]
    new_tags: Vec<String>,

    #[arg(short = 'e', long)]
    description: Option<String>,
}

impl Edit {
    pub fn run(&self) {
        println!("edit");
    }
}
