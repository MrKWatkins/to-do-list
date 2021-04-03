use diesel::{Connection, select, RunQueryDsl, SqliteConnection};
use diesel::sql_types::Integer;
use dotenv::dotenv;
use std::env;
use std::io::stdout;

embed_migrations!("migrations");

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    return SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}",
                                   database_url));
}

pub fn run_migrations(connection: &SqliteConnection)
{
    embedded_migrations::run_with_output(connection, &mut stdout())
        .unwrap_or_else(|_| panic!("Could not apply migrations."));
}

no_arg_sql_function!(last_insert_rowid, Integer);

pub fn get_last_inserted_id(connection: &SqliteConnection) -> i32
{
    return select(last_insert_rowid).first(connection).unwrap();
}
