use crate::view_model::{TaskListViewModel, TaskViewModel};
use crate::model::Task;

#[derive(Clone, Data, Lens)]
pub struct AppViewModel {
    pub today: TaskListViewModel,
    pub this_week: TaskListViewModel,
    pub other: TaskListViewModel,
    pub selected_task: TaskViewModel
}

impl AppViewModel {
    pub fn new(tasks: &[Task]) -> AppViewModel {
        return AppViewModel {
            today: TaskListViewModel::new(tasks),
            this_week: TaskListViewModel::new(&tasks[1..]),
            other: TaskListViewModel::new(&tasks[2..]),
            selected_task: TaskViewModel::none()
        }
    }
}