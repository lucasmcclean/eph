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

#[cfg(test)]
mod tests {
    use super::*;

    fn task(context: &str, priority: Priority, tags: &[&str]) -> Task {
        let tags: Vec<String> = tags.iter().map(|s| s.to_string()).collect();
        Task::new("test".to_string(), context.to_string(), priority).with_tags(tags)
    }

    fn s(raw: &str) -> String {
        raw.to_string()
    }

    #[test]
    fn empty_filter_matches_all() {
        let filter = TaskFilter::new();
        let t = task("work", Priority::High, &[]);
        assert!(filter.matches(&t));
    }

    #[test]
    fn matches_context() {
        let filter = TaskFilter::new().with_contexts([s("work")]);
        assert!(filter.matches(&task("work", Priority::Low, &[])));
        assert!(!filter.matches(&task("personal", Priority::Low, &[])));
    }

    #[test]
    fn matches_priority() {
        let filter = TaskFilter::new().with_priorities([Priority::High]);
        assert!(filter.matches(&task("w", Priority::High, &[])));
        assert!(!filter.matches(&task("w", Priority::Low, &[])));
    }

    #[test]
    fn matches_tag_any() {
        let filter = TaskFilter::new().with_tags([s("urgent")]);
        assert!(filter.matches(&task("w", Priority::High, &["urgent"])));
        assert!(filter.matches(&task("w", Priority::High, &["urgent", "foo"])));
        assert!(!filter.matches(&task("w", Priority::High, &["foo"])));
    }

    #[test]
    fn matches_all_criteria_and() {
        let filter = TaskFilter::new()
            .with_contexts([s("work")])
            .with_priorities([Priority::High])
            .with_tags([s("urgent")]);
        assert!(filter.matches(&task("work", Priority::High, &["urgent"])));
        assert!(!filter.matches(&task("home", Priority::High, &["urgent"])));
        assert!(!filter.matches(&task("work", Priority::Low, &["urgent"])));
        assert!(!filter.matches(&task("work", Priority::High, &["chill"])));
    }

    #[test]
    fn multiple_contexts() {
        let filter = TaskFilter::new().with_contexts([s("work"), s("personal")]);
        assert!(filter.matches(&task("work", Priority::Low, &[])));
        assert!(filter.matches(&task("personal", Priority::Low, &[])));
        assert!(!filter.matches(&task("other", Priority::Low, &[])));
    }

    #[test]
    fn multiple_priorities() {
        let filter = TaskFilter::new().with_priorities([Priority::High, Priority::Medium]);
        assert!(filter.matches(&task("w", Priority::High, &[])));
        assert!(filter.matches(&task("w", Priority::Medium, &[])));
        assert!(!filter.matches(&task("w", Priority::Backlog, &[])));
    }
}
