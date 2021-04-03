use crate::view_model::TaskListViewModel;
use crate::model::Task;

#[derive(Clone, Data, Lens)]
pub struct AppViewModel {
    today: TaskListViewModel
}

impl AppViewModel {
    pub fn new(tasks: &Vec<Task>) -> AppViewModel {
        return AppViewModel {
            today: TaskListViewModel::new(tasks)
        }
    }
}