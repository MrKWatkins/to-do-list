use crate::model::Task;
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct TaskListItemViewModel {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Data, Lens)]
pub struct TaskListViewModel {
    pub items: Arc<Vec<TaskListItemViewModel>>,
}

impl TaskListItemViewModel {
    pub fn new(task: &Task) -> TaskListItemViewModel {
        return TaskListItemViewModel {
            id: task.id,
            name: task.name.clone(),
        };
    }
}

impl TaskListViewModel {
    pub fn new(tasks: &[Task]) -> TaskListViewModel {
        return TaskListViewModel {
            items: Arc::new(tasks.iter().map(|task| TaskListItemViewModel::new(&task)).collect())
        };
    }
}