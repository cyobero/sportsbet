use super::schema::events;

use chrono::NaiveDateTime;
use diesel::Queryable;
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
    pub timestamp: NaiveDateTime,
}
