use uuid::Uuid;

use crate::task::{Priority, Task};

#[derive(Clone, Debug)]
pub enum Patch<T> {
    Keep,
    Set(T),
    Unset,
}

impl<T> Patch<T> {
    fn apply_to(self, target: &mut T) {
        if let Patch::Set(value) = self {
            *target = value;
        }
    }

    fn apply_optional(self, target: &mut Option<T>) {
        match self {
            Patch::Keep => {}
            Patch::Set(value) => *target = Some(value),
            Patch::Unset => *target = None,
        }
    }
}

pub struct TaskPatch {
    id: Uuid,
    pub title: Patch<String>,
    pub context: Patch<String>,
    pub priority: Patch<Priority>,
    pub tags: Patch<Vec<String>>,
    pub description: Patch<String>,
}

impl TaskPatch {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn apply_to(self, task: &mut Task) {
        self.title.apply_to(&mut task.title);
        self.context.apply_to(&mut task.context);
        self.priority.apply_to(&mut task.priority);
        self.tags.apply_to(&mut task.tags);
        self.description.apply_optional(&mut task.description);
    }
}
