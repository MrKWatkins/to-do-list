use crate::model::Task;

#[derive(Clone, Data, Lens)]
pub struct TaskViewModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

impl TaskViewModel {
    pub fn new(task: &Task) -> TaskViewModel {
        return TaskViewModel {
            id: task.id,
            name: task.name.clone(),
            description: None
        };
    }
}