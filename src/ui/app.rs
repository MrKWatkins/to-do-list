use crate::view_model::AppViewModel;
use relm::{connect, Relm, Update, Widget, init};
use relm_derive::Msg;
use gtk::prelude::*;
use gtk::{Orientation, Window, Inhibit, WindowType, Box as GtkBox, ContainerExt};
use crate::ui::task_list::TaskList;

pub fn run(view_model: AppViewModel) -> Result<(), ()> {
    return App::run(view_model);
}

#[derive(Msg)]
pub enum AppMessage
{
    Quit,
}

pub struct App
{
    model: AppViewModel,
    window: Window
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
        let window = Window::new(WindowType::Toplevel);

        connect!(relm, window, connect_delete_event(_, _), return (Some(AppMessage::Quit), Inhibit(false)));

        let container = GtkBox::new(Orientation::Horizontal, 0);

        container.add(init::<TaskList>(model.today.clone()).expect("Today List").widget());
        container.add(init::<TaskList>(model.this_week.clone()).expect("This Week List").widget());
        container.add(init::<TaskList>(model.other.clone()).expect("Other List").widget());

        window.add(&container);

        window.show_all();

        return App {
            model,
            window,
        };
    }
}