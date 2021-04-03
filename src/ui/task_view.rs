use crate::view_model::TaskViewModel;
use druid::widget::{List, Scroll, Flex, Label};
use druid::{Widget, WidgetExt};

pub fn create_task_view() -> impl Widget<Option<TaskViewModel>>
{
    return
        Flex::column()
            .with_child(create_row("Name", |vm| &vm.name));
}

fn create_row(label: &str, get_text: impl Fn(&TaskViewModel) -> &str + 'static) -> impl Widget<Option<TaskViewModel>>
{
    return Flex::row()
        .with_child(Label::new(label))
        .with_child(Label::dynamic(move |task: &Option<TaskViewModel>, _| task.as_ref().map_or("", |vm| get_text(&vm)).to_string()));
}