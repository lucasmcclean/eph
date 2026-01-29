use clap::Parser;

use crate::priority::Priority;

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
#[command(
    group(
        clap::ArgGroup::new("changes")
            .required(true)
            .args([
                "title",
                "new_context",
                "priority",
                "add_tags",
                "delete_tags",
                "new_tags",
                "description",
            ])
    )
)]
pub struct Edit {
    // Either the runtime ID or title (fuzzy)
    identifier: String,

    #[arg(short = 'f', long, default_value = "std")]
    context: String,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(short = 'c', long)]
    new_context: Option<String>,

    #[arg(short, long, help = "Priority: 1–4 (high, medium, low, backlog)")]
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
        println!("Editing {}", self.identifier);
        if let Some(title) = self.title.as_deref() {
            println!("- New title: {}", title);
        }
        if let Some(context) = self.new_context.as_deref() {
            println!("- New context: {}", context);
        }
        if let Some(priority) = self.priority {
            println!("- New priority: {}", priority);
        }
        if !self.add_tags.is_empty() {
            println!("- Added tags: {:?}", self.add_tags);
        }
        if !self.delete_tags.is_empty() {
            println!("- Deleted tags: {:?}", self.delete_tags);
        }
        if !self.new_tags.is_empty() {
            println!("- New tags: {:?}", self.new_tags);
        }
        if let Some(description) = self.description.as_deref() {
            println!("- New description: {}", description);
        }
    }
}
