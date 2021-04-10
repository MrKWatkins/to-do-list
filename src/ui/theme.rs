use druid::{Key, Env, Color};
use crate::view_model::AppViewModel;
use druid::theme::*;

pub const TEXT_COLOUR: Key<Color> = Key::new("com.mrkwatkins.to-do-list.text-colour");
pub const TEXT_DISABLED_COLOUR: Key<Color> = Key::new("com.mrkwatkins.to-do-list.text-disabled-colour");

pub fn configure_theme(env: &mut Env, _app: &AppViewModel)
{
    let label_colour: Color = env.get(LABEL_COLOR);
    let label_colour_rgba = label_colour.as_rgba();
    let disabled_colour = Color::rgb(label_colour_rgba.0 * 0.7, label_colour_rgba.1 * 0.7, label_colour_rgba.2 * 0.7);

    env.set(TEXT_COLOUR, label_colour);
    env.set(TEXT_DISABLED_COLOUR, disabled_colour);
}