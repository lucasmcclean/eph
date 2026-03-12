use clap::Parser;
use uuid::Uuid;

use crate::{storage, task::Priority};

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
                "delete_description",
            ])
    )
)]
pub struct Edit {
    id: Uuid,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    context: Option<String>,

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

    #[arg(
        short = 'r',
        long,
        value_delimiter = ',',
        conflicts_with = "description"
    )]
    delete_description: bool,
}

impl Edit {
    pub fn run(&self) {
        let mut tasks = storage::load(storage::DataPath::default()).unwrap();

        let edited_task = tasks.iter_mut().find(|task| task.id() == self.id);

        match edited_task {
            Some(task) => {
                if let Some(title) = &self.title {
                    task.title = title.clone();
                }

                if let Some(context) = &self.context {
                    task.context = context.clone();
                }

                if let Some(priority) = &self.priority {
                    task.priority = *priority;
                }

                if let Some(description) = &self.description {
                    task.description = Some(description.clone());
                }

                if self.delete_description {
                    task.description = None;
                }

                if !self.new_tags.is_empty() {
                    task.tags = self.new_tags.clone();
                } else {
                    task.tags.extend(self.add_tags.iter().cloned());

                    task.tags.retain(|tag| !self.delete_tags.contains(tag));
                }
                storage::store(storage::DataPath::default(), &tasks).expect("Failed to save tasks");
                println!("Successfully edited");
            }
            None => println!("Item not found"),
        }
    }
}
