#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::model::Task;
use crate::view_model::AppViewModel;

mod schema;
mod db;
mod ui;
mod model;
mod view_model;

fn main() {
    println!("Opening connection...");

    let connection = db::establish_connection();

    println!("Applying migrations...");

    db::run_migrations(&connection);

    println!("Done!");

    let tasks = Task::get_open(&connection);

    let view_model = AppViewModel::new(&tasks);

    ui::run(view_model)
        .unwrap_or_else(|_| panic!("Error launching UI"));
}
