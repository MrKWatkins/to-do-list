use crate::view_model::{TaskListViewModel, TaskViewModel};
use druid::widget::{Label, List, Scroll};
use druid::{UnitPoint, WidgetExt, Widget};

pub fn create_task_list() -> impl Widget<TaskListViewModel>
{
    return Scroll::new(
        List::new(create_list_item))
        .vertical()
        .lens(TaskListViewModel::items);
}

fn create_list_item() -> impl Widget<TaskViewModel>
{
    return Label::dynamic(|name: &String, _| name.to_string()).lens(TaskViewModel::name)
        .align_vertical(UnitPoint::LEFT)
        .padding(10.0)
        .expand_width();
}