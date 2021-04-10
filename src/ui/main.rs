use crate::model::Task;
use crate::view_model::AppViewModel;
use crate::ui::{create_app, configure_theme};
use druid::{PlatformError, WindowDesc, AppLauncher};

pub fn launch(tasks: &[Task]) -> Result<(), PlatformError> {

    let app = AppViewModel::new(tasks);

    let window = WindowDesc::new(create_app).title("To Do List");

    AppLauncher::with_window(window)
        .configure_env(configure_theme)
        .use_simple_logger()
        .launch(app)?;

    return Ok(());
}