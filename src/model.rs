use super::schema::events;

use chrono::NaiveDateTime;
use diesel::sql_types::{Float4, Integer, Varchar};
use diesel::{Queryable, QueryableByName};
use diesel_derive_enum::DbEnum;
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
