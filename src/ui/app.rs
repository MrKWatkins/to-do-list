use crate::view_model::AppViewModel;
use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Padding, Label};
use crate::ui::{create_task_list, create_task_view};

pub fn create_app() -> impl Widget<AppViewModel>
{
    return
        Padding::new(
            10.0,
            Flex::column()
                .with_flex_child(
                    Flex::row()
                .with_flex_child(
                    Flex::column()
                        .with_child(Label::new("Today"))
                        .with_flex_child(create_task_list().lens(AppViewModel::today), 1.0),
                    1.0)
                .with_flex_child(
                    Flex::column()
                        .with_child(Label::new("This Week"))
                        .with_flex_child(create_task_list().lens(AppViewModel::this_week), 1.0),
                    1.0)
                .with_flex_child(
                    Flex::column()
                        .with_child(Label::new("Other"))
                        .with_flex_child(create_task_list().lens(AppViewModel::other), 1.0),
                    1.0), 1.0)
                .with_child(create_task_view().lens(AppViewModel::selected_task)));
}