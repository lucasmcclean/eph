use crate::task::{Priority, Task};

#[derive(Default)]
pub struct TaskFilter {
    contexts: Vec<String>,
    priorities: Vec<Priority>,
    tags: Vec<String>,
}

impl TaskFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_contexts(mut self, contexts: impl AsRef<[String]>) -> Self {
        self.contexts = contexts.as_ref().to_vec();
        self
    }

    pub fn with_priorities(mut self, priorities: impl AsRef<[Priority]>) -> Self {
        self.priorities = priorities.as_ref().to_vec();
        self
    }

    pub fn with_tags(mut self, tags: impl AsRef<[String]>) -> Self {
        self.tags = tags.as_ref().to_vec();
        self
    }

    pub fn matches(&self, task: &Task) -> bool {
        (self.contexts.is_empty() || self.contexts.contains(&task.context))
            && (self.priorities.is_empty() || self.priorities.contains(&task.priority))
            && (self.tags.is_empty() || task.tags.iter().any(|t| self.tags.contains(t)))
    }
}
