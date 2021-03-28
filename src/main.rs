#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod schema;
mod db;

fn main() {
    println!("Opening connection...");

    let connection = db::establish_connection();

    println!("Applying migrations...");

    db::run_migrations(&connection);

    println!("Done!");
}
