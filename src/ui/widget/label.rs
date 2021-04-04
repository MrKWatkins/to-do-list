use druid::{Widget, Color};
use druid::widget::{Either, Label};

pub fn disableable_label(text: &str) -> impl Widget<bool>
{
    return Either::<bool>::new(
        |data, _| *data,
        Label::new(text),
        Label::new(text).with_text_color(Color::rgb8(0x44, 0x44, 0x44)));
}