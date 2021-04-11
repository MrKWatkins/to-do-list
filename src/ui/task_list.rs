use crate::view_model::{TaskListViewModel};
use gtk::{ListBox, ListBoxExt, ListBoxRowBuilder, ContainerExt, WidgetExt, BoxExt, Orientation, Label};
use relm::{connect, Relm};
use crate::ui::{App, AppMessage};

pub fn initialize_task_list(relm: &Relm<App>, list_box: &mut ListBox, view_model: &TaskListViewModel)
{
    for task in &view_model.tasks
    {
        let row = ListBoxRowBuilder::new()
            .name(&*task.name)
            .build();

        let row_box = gtk::Box::new(Orientation::Horizontal, 10);
        row_box.pack_start(&Label::new(Some(&*task.name)), true, true, 0);
        row.add(&row_box);

        list_box.add(&row);
    }

    let list_type = view_model.list_type.clone();

    // Only fire the event if we have a row; None for row indicates selection has been cleared.
    connect!(relm, list_box, connect_row_selected(_, row), return (row.map(|_| AppMessage::TaskSelected(list_type.clone())), ()));

    list_box.show_all();
}