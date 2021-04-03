use crate::db::get_last_inserted_id;
use crate::schema::tasks;
use crate::schema::tasks::columns;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::insert_into;
use diesel::prelude::*;

#[derive(Queryable, PartialEq, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub deadline: Option<NaiveDate>,
    pub completed: Option<NaiveDateTime>
}

impl Task {
    pub fn get_open(connection: &SqliteConnection) -> Vec<Task>
    {
        return tasks::dsl::tasks
            .filter(columns::completed.is_null())
            .load::<Task>(connection)
            .expect("Error retrieving open tasks.");
    }

    pub fn get_by_id(connection: &SqliteConnection, id: i32) -> Task
    {
        return tasks::dsl::tasks
            .filter(columns::id.eq(id))
            .first::<Task>(connection)
            .expect(&format!("Error retrieving task with ID {}.", id));
    }
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct TaskModification<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub deadline: Option<&'a NaiveDate>,
    pub completed: Option<&'a NaiveDateTime>,
}

impl TaskModification<'_> {
    pub fn insert<'a>(connection: &SqliteConnection, task: &TaskModification<'a>) -> Task
    {
        insert_into(tasks::table)
            .values(task)
            .execute(connection)
            .expect("Error inserting task.");

        let id = get_last_inserted_id(connection);

        return Task::get_by_id(&connection, id);
    }
}