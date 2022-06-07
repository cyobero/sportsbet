use super::db::{Creatable, Deletable, Retrievable};
use super::schema::events::{self, dsl};

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::{result, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
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

#[derive(Clone, Copy, Serialize)]
pub struct EventQuery {
    pub id: Option<i32>,
    pub odds: Option<i32>,
}

impl Deletable for Event {
    fn delete(&self, conn: &PgConnection) -> Result<Event, result::Error> {
        diesel::delete(dsl::events.filter(dsl::id.eq(&self.id))).get_result(conn)
    }
}

impl Retrievable<EventQuery> for Event {
    fn query(conn: &PgConnection, data: &EventQuery) -> Result<Vec<Event>, result::Error> {
        match data {
            EventQuery {
                id: Some(_id),
                odds: None,
            } => dsl::events.filter(dsl::id.eq(_id)).get_results(conn),

            EventQuery {
                id: None,
                odds: Some(_odds),
            } => dsl::events.filter(dsl::odds.eq(_odds)).get_results(conn),

            EventQuery {
                id: Some(_id),
                odds: Some(_odds),
            } => dsl::events
                .filter(dsl::id.eq(_id))
                .filter(dsl::odds.eq(_odds))
                .get_results(conn),

            EventQuery {
                id: None,
                odds: None,
            } => dsl::events.load(conn),
        }
    }

    fn all(conn: &PgConnection) -> Result<Vec<Event>, result::Error> {
        dsl::events.limit(100).load(conn)
    }
}

impl Creatable for NewEvent {
    type Output = Event;
    fn create(&self, conn: &PgConnection) -> Result<Event, result::Error> {
        diesel::insert_into(dsl::events)
            .values(self)
            .get_result(conn)
    }
}
