use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::Priority;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Task {
    // What's not serializing?
    id: Uuid,
    pub title: String,
    pub context: String,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, context: String, priority: Priority) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            context,
            priority,
            tags: Vec::new(),
            description: None,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn _with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_description_opt(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} ({})", self.priority, self.title, self.context)?;

        for tag in &self.tags {
            write!(f, " #{}", tag)?;
        }

        if let Some(description) = &self.description {
            write!(f, "\n\t{}", description)?;
        }

        Ok(())
    }
}
