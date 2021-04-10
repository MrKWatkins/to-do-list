use crate::model::Task;

#[derive(Clone)]
pub struct TaskListItemViewModel {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct TaskListViewModel {
    pub name: String,
    pub items: Vec<TaskListItemViewModel>,
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
    pub fn new(name: &str, tasks: &[Task]) -> TaskListViewModel {
        return TaskListViewModel {
            name: name.to_string(),
            items: tasks.iter().map(|task| TaskListItemViewModel::new(&task)).collect()
        };
    }
}