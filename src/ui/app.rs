use crate::view_model::AppViewModel;
use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Padding};
use crate::ui::create_task_list;

pub fn create_app() -> impl Widget<AppViewModel>
{
    return
        Padding::new(
            10.0,
            Flex::row()
                .with_flex_child(
                    create_task_list().lens(AppViewModel::today), 1.0));
}