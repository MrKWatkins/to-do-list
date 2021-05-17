use crate::view_model::{AppViewModel, TaskListType};
use crate::ui::task_list::TaskList;
use orbtk::prelude::*;
use orbtk::prelude::Widget;

pub fn run(view_model: AppViewModel) {
    //return App::run(view_model);
    return Application::new()
        .window(|ctx| {
            Window::new()
                .title("To Do List")
                .position((100.0, 100.0))
                .size(420.0, 240.0)
                .resizeable(true)
                .child(
                    Grid::new()
                        .columns(Columns::create().push("auto").push("auto").push("auto"))
                        .rows(Rows::create().push("auto").push("auto"))
                        .v_align("start")
                        .h_align("start")
                        .child(
                            TaskList::new()
                                .title("Today")
                                .attach(Grid::column(0))
                                .attach(Grid::row(0))
                                .build(ctx)
                        )
                        .child(
                            TaskList::new()
                                .title("This Week")
                                .attach(Grid::column(1))
                                .attach(Grid::row(0))
                                .build(ctx)
                        )
                        .child(
                            TaskList::new()
                                .title("Other")
                                .attach(Grid::column(2))
                                .attach(Grid::row(0))
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
/*
#[derive(Msg)]
pub enum AppMessage
{
    TaskSelected(TaskListType),
    Quit,
}

pub struct App
{
    model: AppViewModel,
    window: Window,
    today: ListBox,
    this_week: ListBox,
    other: ListBox,
}

impl Update for App {
    type Model = AppViewModel;
    type ModelParam = AppViewModel;
    type Msg = AppMessage;

    fn model(_relm: &Relm<Self>, param: AppViewModel) -> AppViewModel {
        return param;
    }

    fn update(&mut self, event: AppMessage) {
        match event {
            AppMessage::TaskSelected(list_type) => {
                match list_type {
                    TaskListType::Today => {
                        println!("Task Selected in Today");
                        self.this_week.unselect_all();
                        self.other.unselect_all();
                    }
                    TaskListType::ThisWeek => {
                        println!("Task Selected in ThisWeek");
                        self.today.unselect_all();
                        self.other.unselect_all();
                    }
                    TaskListType::Other => {
                        println!("Task Selected in Other");
                        self.today.unselect_all();
                        self.this_week.unselect_all();
                    }
                }
            },
            AppMessage::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn root(&self) -> Window {
        return self.window.clone();
    }

    fn view(relm: &Relm<Self>, model: AppViewModel) -> App {
        let glade = include_str!("app.glade");
        let builder = Builder::from_string(glade);

        let window: Window = builder.get_object("window").unwrap();
        connect!(relm, window, connect_delete_event(_, _), return (Some(AppMessage::Quit), Inhibit(false)));

        let mut today: ListBox = builder.get_object("today-list").unwrap();
        initialize_task_list(relm, &mut today, &model.today);

        let mut this_week: ListBox = builder.get_object("this-week-list").unwrap();
        initialize_task_list(relm, &mut this_week, &model.this_week);

        let mut other: ListBox = builder.get_object("other-list").unwrap();
        initialize_task_list(relm, &mut other, &model.other);

        window.show_all();

        return App {
            model,
            window,
            today,
            this_week,
            other
        };
    }
}

*/