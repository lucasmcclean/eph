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
                target.extend(add);
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
