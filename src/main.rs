#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate druid;

use crate::model::Task;

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

    ui::launch(&tasks)
        .unwrap_or_else(|_| panic!("Error launching UI"));
}
