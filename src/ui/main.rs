use druid::{PlatformError, WindowDesc, AppLauncher, Widget};
use druid::widget::Label;

pub fn launch() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build);

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(())?;

    return Ok(());
}

fn build() -> impl Widget<()> {
    return Label::new("TODO");
}