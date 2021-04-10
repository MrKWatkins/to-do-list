use crate::model::Task;
use crate::ui::form::FormField;

#[derive(Clone)]
pub struct TaskViewModel {
    pub id: i32,
    pub name: FormField<String>
}

impl TaskViewModel {
    pub fn new(task: &Task) -> TaskViewModel {
        return TaskViewModel {
            id: task.id,
            name: FormField::new("Name", task.name.clone())
        };
    }

    pub fn none() -> TaskViewModel {
        return TaskViewModel {
            id: 0,
            name: FormField::disabled("Name", "".to_string())
        };
    }
}