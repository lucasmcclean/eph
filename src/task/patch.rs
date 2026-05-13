use uuid::Uuid;

use crate::task::{Priority, Task};

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Patch<T> {
    Keep,
    Set(T),
    Unset,
}

impl<T> Patch<T> {
    pub fn apply_to(self, target: &mut T) {
        if let Patch::Set(value) = self {
            *target = value;
        }
    }

    pub fn apply_optional(self, target: &mut Option<T>) {
        match self {
            Patch::Keep => {}
            Patch::Set(value) => *target = Some(value),
            Patch::Unset => *target = None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum VecPatch<T> {
    Keep,
    Set(Vec<T>),
    Unset,
    Modify { add: Vec<T>, remove: Vec<T> },
}

impl<T: PartialEq> VecPatch<T> {
    pub fn apply_to(self, target: &mut Vec<T>) {
        match self {
            VecPatch::Keep => {}
            VecPatch::Set(v) => *target = v,
            VecPatch::Unset => target.clear(),
            VecPatch::Modify { add, remove } => {
                let to_add: Vec<_> = add
                    .into_iter()
                    .filter(|item| !target.contains(item))
                    .collect();
                target.extend(to_add);
                target.retain(|item| !remove.contains(item));
            }
        }
    }
}

pub struct TaskPatch {
    id: Uuid,
    pub title: Patch<String>,
    pub context: Patch<String>,
    pub priority: Patch<Priority>,
    pub tags: VecPatch<String>,
    pub description: Patch<String>,
}

impl TaskPatch {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            title: Patch::Keep,
            context: Patch::Keep,
            priority: Patch::Keep,
            tags: VecPatch::Keep,
            description: Patch::Keep,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(self, value: Option<String>) -> Self {
        Self {
            title: value.map_or(Patch::Keep, Patch::Set),
            ..self
        }
    }

    pub fn context(self, value: Option<String>) -> Self {
        Self {
            context: value.map_or(Patch::Keep, Patch::Set),
            ..self
        }
    }

    pub fn priority(self, value: Option<Priority>) -> Self {
        Self {
            priority: value.map_or(Patch::Keep, Patch::Set),
            ..self
        }
    }

    pub fn tags(self, value: VecPatch<String>) -> Self {
        Self {
            tags: value,
            ..self
        }
    }

    pub fn description(self, value: Patch<String>) -> Self {
        Self {
            description: value,
            ..self
        }
    }

    pub fn apply_to(self, task: &mut Task) {
        self.title.apply_to(&mut task.title);
        self.context.apply_to(&mut task.context);
        self.priority.apply_to(&mut task.priority);
        self.description.apply_optional(&mut task.description);
        self.tags.apply_to(&mut task.tags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_keep_does_nothing() {
        let mut value = 42;
        Patch::<i32>::Keep.apply_to(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn patch_set_changes_value() {
        let mut value = 42;
        Patch::Set(100).apply_to(&mut value);
        assert_eq!(value, 100);
    }

    #[test]
    fn patch_optional_keep() {
        let mut value = Some(42);
        Patch::<i32>::Keep.apply_optional(&mut value);
        assert_eq!(value, Some(42));
    }

    #[test]
    fn patch_optional_set() {
        let mut value: Option<i32> = None;
        Patch::Set(42).apply_optional(&mut value);
        assert_eq!(value, Some(42));
    }

    #[test]
    fn patch_optional_unset() {
        let mut value = Some(42);
        Patch::<i32>::Unset.apply_optional(&mut value);
        assert_eq!(value, None);
    }

    #[test]
    fn vec_patch_keep() {
        let mut v = vec![1, 2, 3];
        VecPatch::<i32>::Keep.apply_to(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn vec_patch_set() {
        let mut v = vec![1, 2, 3];
        VecPatch::Set(vec![4, 5]).apply_to(&mut v);
        assert_eq!(v, vec![4, 5]);
    }

    #[test]
    fn vec_patch_unset() {
        let mut v = vec![1, 2, 3];
        VecPatch::<i32>::Unset.apply_to(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn vec_patch_modify_add_new() {
        let mut v = vec!["a".to_string(), "b".to_string()];
        VecPatch::Modify {
            add: vec!["c".to_string()],
            remove: vec![],
        }
        .apply_to(&mut v);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn vec_patch_modify_no_duplicate_on_add_existing() {
        let mut v = vec!["a".to_string(), "b".to_string()];
        VecPatch::Modify {
            add: vec!["a".to_string()],
            remove: vec![],
        }
        .apply_to(&mut v);
        assert_eq!(v, vec!["a", "b"]);
    }

    #[test]
    fn vec_patch_modify_remove() {
        let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        VecPatch::Modify {
            add: vec![],
            remove: vec!["b".to_string()],
        }
        .apply_to(&mut v);
        assert_eq!(v, vec!["a", "c"]);
    }

    #[test]
    fn vec_patch_modify_add_and_remove_same() {
        let mut v = vec!["a".to_string()];
        VecPatch::Modify {
            add: vec!["a".to_string()],
            remove: vec!["a".to_string()],
        }
        .apply_to(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn vec_patch_modify_add_and_remove_different() {
        let mut v = vec!["a".to_string()];
        VecPatch::Modify {
            add: vec!["b".to_string()],
            remove: vec!["a".to_string()],
        }
        .apply_to(&mut v);
        assert_eq!(v, vec!["b"]);
    }

    #[test]
    fn task_patch_applies_title() {
        let mut task = Task::new("old".to_string(), "ctx".to_string(), Priority::Low);
        let id = task.id();
        let patch = TaskPatch::new(id).title(Some("new".to_string()));
        patch.apply_to(&mut task);
        assert_eq!(task.title, "new");
    }

    #[test]
    fn task_patch_applies_priority() {
        let mut task = Task::new("t".to_string(), "ctx".to_string(), Priority::Low);
        let id = task.id();
        let patch = TaskPatch::new(id).priority(Some(Priority::High));
        patch.apply_to(&mut task);
        assert_eq!(task.priority, Priority::High);
    }

    #[test]
    fn task_patch_keeps_unchanged_fields() {
        let mut task = Task::new("t".to_string(), "ctx".to_string(), Priority::Low);
        let id = task.id();
        let patch = TaskPatch::new(id);
        patch.apply_to(&mut task);
        assert_eq!(task.title, "t");
        assert_eq!(task.context, "ctx");
        assert_eq!(task.priority, Priority::Low);
    }
}
