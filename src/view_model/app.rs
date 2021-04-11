use crate::view_model::{TaskListViewModel, TaskViewModel, TaskListType};
use crate::model::Task;

#[derive(Clone)]
pub struct AppViewModel {
    pub today: TaskListViewModel,
    pub this_week: TaskListViewModel,
    pub other: TaskListViewModel,
    pub selected_task: TaskViewModel
}

impl AppViewModel {
    pub fn new(tasks: &[Task]) -> AppViewModel {
        return AppViewModel {
            today: TaskListViewModel::new(TaskListType::Today, tasks),
            this_week: TaskListViewModel::new(TaskListType::ThisWeek, &tasks[1..]),
            other: TaskListViewModel::new(TaskListType::Other, &tasks[2..]),
            selected_task: TaskViewModel::none()
        }
    }
}