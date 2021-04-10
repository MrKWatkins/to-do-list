use crate::view_model::TaskListViewModel;
use gtk::{Orientation, Label, Box as GtkBox, ContainerExt};
use relm::{Widget, Update, Relm};
use relm_derive::Msg;


#[derive(Msg)]
pub enum TaskListMessage
{
}

pub struct TaskList
{
    model: TaskListViewModel,
    container: GtkBox
}

impl Update for TaskList {
    type Model = TaskListViewModel;
    type ModelParam = TaskListViewModel;
    type Msg = TaskListMessage;

    fn model(_relm: &Relm<Self>, param: Self::ModelParam) -> TaskListViewModel {
        return param;
    }

    fn update(&mut self, _event: TaskListMessage) {
    }
}

impl Widget for TaskList {
    type Root = GtkBox;

    fn root(&self) -> GtkBox {
        return self.container.clone();
    }

    fn view(_relm: &Relm<Self>, model: TaskListViewModel) -> TaskList {
        let container = GtkBox::new(Orientation::Vertical, 0);

        let label = Label::new(Some(&*model.name));

        container.add(&label);

        return TaskList {
            model,
            container,
        };
    }
}