use super::db::{Creatable, Retrievable};
use super::schema::events;

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::{result, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Queryable)]
pub struct Event {
    pub id: i32,
    pub description: String,
    pub odds: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub description: String,
    pub odds: i32,
}

impl Creatable for NewEvent {
    type Output = Event;
    fn create(&self, conn: &PgConnection) -> Result<Event, result::Error> {
        diesel::insert_into(events::dsl::events)
            .values(self)
            .get_result(conn)
    }
}
