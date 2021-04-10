use crate::view_model::TaskViewModel;
use druid::widget::{Flex};
use druid::{Widget, WidgetExt, Lens, Data};
use crate::ui::widget::{FormLabel, FormField, FormTextBox};

pub fn create_task_view() -> impl Widget<TaskViewModel>
{
    return
        Flex::column()
            .with_child(create_row(TaskViewModel::name));
}

fn create_row(lens: impl Lens<TaskViewModel, FormField<String>>) -> impl Widget<TaskViewModel>
{
    return Flex::row()
        .with_child(FormLabel::new())
        .with_child(FormTextBox::new())
        .lens(lens);
}