use crate::view_model::TaskViewModel;
use druid::widget::{Flex, TextBox};
use druid::{Widget, WidgetExt};
use crate::ui::lens::{OptionLens, IsSomeLens};
use crate::ui::widget::disableable_label;

pub fn create_task_view() -> impl Widget<Option<TaskViewModel>>
{
    return
        Flex::column()
            .with_child(create_row("Name"));
}

fn create_row(text: &str) -> impl Widget<Option<TaskViewModel>>
{
    let label = disableable_label(text).lens(IsSomeLens::new());
    let text_box = TextBox::new().lens(OptionLens::new(TaskViewModel::name, "".to_string()));
    return Flex::row()
        .with_child(label)
        .with_child(text_box);
}