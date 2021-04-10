use crate::view_model::AppViewModel;
use relm::{connect, Relm, Update, Widget, init};
use relm_derive::Msg;
use gtk::prelude::*;
use gtk::{Orientation, Window, Inhibit, WindowType, Box as GtkBox, ContainerExt, Builder};
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
        let glade = include_str!("app.glade");
        let builder = Builder::from_string(glade);

        let window: Window = builder.get_object("window").unwrap();

        connect!(relm, window, connect_delete_event(_, _), return (Some(AppMessage::Quit), Inhibit(false)));

        window.show_all();

        return App {
            model,
            window,
        };
    }
}