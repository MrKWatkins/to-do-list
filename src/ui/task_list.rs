use crate::view_model::{TaskListViewModel};
use orbtk::prelude::*;

const TASK_LIST_ID: &str = "task_list";

pub enum TaskListAction
{

}

#[derive(Default, AsAny)]
pub struct TaskListState
{
    task_list: Entity
}

impl State for TaskListState
{
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.task_list = ctx
            .entity_of_child(TASK_LIST_ID)
            .expect("Task list could not be found!");
    }

    fn update(&mut self, reg: &mut Registry, ctx: &mut Context) {
    }
}

widget!(TaskList<TaskListState> {
    title: String16
});

impl Template for TaskList {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TaskList")
            .title("")
            .child(
                Stack::new()
                    .margin((10.0, 10.0, 10.0, 10.0))
                    .h_align("center")
                    .child(
                        TextBlock::new()
                            .text(("title", id))
                            .margin((0.0, 0.0, 0.0, 8.0))
                            .build(ctx),
                    )
                    .child(
                        ListView::new()
                            .id(TASK_LIST_ID)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}