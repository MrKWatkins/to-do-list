use crate::model::Task;
use crate::view_model::AppViewModel;
use crate::ui::create_app;
use druid::{PlatformError, WindowDesc, AppLauncher};

pub fn launch(tasks: &Vec<Task>) -> Result<(), PlatformError> {

    let app = AppViewModel::new(tasks);

    let window = WindowDesc::new(create_app);

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(app)?;

    return Ok(());
}