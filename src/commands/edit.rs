use clap::Parser;
use uuid::Uuid;

use crate::{
    app::{self, EditStatus},
    task::{Patch, Priority, TaskPatch, VecPatch},
};

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
#[command(
    group(
        clap::ArgGroup::new("changes")
            .required(true)
            .multiple(true)
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
        let tags = if !self.new_tags.is_empty() {
            VecPatch::Set(self.new_tags.clone())
        } else if !self.add_tags.is_empty() || !self.delete_tags.is_empty() {
            VecPatch::Modify {
                add: self.add_tags.clone(),
                remove: self.delete_tags.clone(),
            }
        } else {
            VecPatch::Keep
        };

        let description = if self.delete_description {
            Patch::Unset
        } else {
            self.description.clone().map_or(Patch::Keep, Patch::Set)
        };

        let patch = TaskPatch::new(self.id)
            .title(self.title.clone())
            .context(self.context.clone())
            .priority(self.priority)
            .description(description)
            .tags(tags);

        match app::edit_task(patch) {
            EditStatus::Updated => println!("Task updated."),
            EditStatus::NotFound => eprintln!("Task not found."),
            EditStatus::StorageError(err) => {
                eprintln!("Failed to save task: {}", err)
            }
        }
    }
}
