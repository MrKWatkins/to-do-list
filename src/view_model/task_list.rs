use crate::model::Task;
use crate::view_model::TaskViewModel;
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
pub struct TaskListViewModel {
    pub items: Arc<Vec<TaskViewModel>>,
}

impl TaskListViewModel {
    pub fn new(tasks: &[Task]) -> TaskListViewModel {
        return TaskListViewModel {
            items: Arc::new(tasks.iter().map(|task| TaskViewModel::new(&task)).collect())
        };
    }
}