use crate::view_model::TaskViewModel;
use druid::widget::{Flex, Label, TextBox};
use druid::{Widget, WidgetExt};
use crate::ui::OptionLens;

pub fn create_task_view() -> impl Widget<Option<TaskViewModel>>
{
    return
        Flex::column()
            .with_child(create_row("Name").lens(OptionLens::new(TaskViewModel::name, "".to_string())));
}

fn create_row(label: &str) -> impl Widget<String>
{
    return Flex::row()
        .with_child(Label::new(label))
        .with_child(TextBox::new());
}