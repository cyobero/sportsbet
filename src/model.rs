use super::db::{Creatable, Deletable, Retrievable};
use super::schema::events::{self, dsl as events_dsl};
use super::schema::games::{self, dsl as games_dsl};

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Queryable)]
pub struct Game {
    pub id: i32,
    pub home: String,
    pub away: String,
    pub start: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub home: String,
    pub away: String,
    pub start: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Event {
    pub id: i32,
    pub description: String,
    pub odds: i32,
    pub result_id: Option<i32>,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
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

impl Creatable for NewGame {
    type Output = Game;
    fn create(&self, conn: &PgConnection) -> Result<Game, DieselError> {
        diesel::insert_into(games_dsl::games)
            .values(self)
            .get_result(conn)
    }
}

impl Deletable for Event {
    fn delete(&self, conn: &PgConnection) -> Result<Event, DieselError> {
        diesel::delete(events_dsl::events.filter(events_dsl::id.eq(&self.id))).get_result(conn)
    }
}

impl Retrievable<EventQuery> for Event {
    fn query(conn: &PgConnection, data: &EventQuery) -> Result<Vec<Event>, DieselError> {
        match data {
            EventQuery {
                id: Some(_id),
                odds: None,
            } => events_dsl::events
                .filter(events_dsl::id.eq(_id))
                .get_results(conn),

            EventQuery {
                id: None,
                odds: Some(_odds),
            } => events_dsl::events
                .filter(events_dsl::odds.eq(_odds))
                .get_results(conn),

            EventQuery {
                id: Some(_id),
                odds: Some(_odds),
            } => events_dsl::events
                .filter(events_dsl::id.eq(_id))
                .filter(events_dsl::odds.eq(_odds))
                .get_results(conn),

            EventQuery {
                id: None,
                odds: None,
            } => events_dsl::events.load(conn),
        }
    }

    fn all(conn: &PgConnection) -> Result<Vec<Event>, DieselError> {
        events_dsl::events
            .limit(100)
            .order_by(events_dsl::timestamp.desc())
            .load(conn)
    }
}

impl Creatable for NewEvent {
    type Output = Event;
    fn create(&self, conn: &PgConnection) -> Result<Event, DieselError> {
        diesel::insert_into(events_dsl::events)
            .values(self)
            .get_result(conn)
    }
}
