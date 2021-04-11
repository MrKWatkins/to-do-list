use crate::model::Task;

#[derive(Clone)]
pub enum TaskListType
{
    Today,
    ThisWeek,
    Other
}

#[derive(Clone)]
pub struct TaskListItemViewModel {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct TaskListViewModel {
    pub list_type: TaskListType,
    pub tasks: Vec<TaskListItemViewModel>,
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
    pub fn new(list_type: TaskListType, tasks: &[Task]) -> TaskListViewModel {
        return TaskListViewModel {
            list_type,
            tasks: tasks.iter().map(|task| TaskListItemViewModel::new(&task)).collect()
        };
    }
}