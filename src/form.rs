//! This module contains structs for handling form input.

use super::db::Creatable;
use super::schema::events;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameForm {
    pub home: String,
    pub away: String,
    pub start: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "events"]
pub struct EventForm {
    pub game_id: i32,
    pub description: String,
    pub odds: i32,
}

impl GameForm {
    pub fn new() -> Self {
        GameForm {
            home: "HOME".to_owned(),
            away: "AWAY".to_owned(),
            start: "1987-10-03T17:00:00".to_owned(),
        }
    }

    /// Return a NaiveDateTime made from `GameForm`'s `start` String.
    pub fn start_to_naive(&self) -> NaiveDateTime {
        let start = self.start.as_bytes();
        let (yr, mo, dy, hr, mn) = (
            String::from_utf8(start[0..4].to_vec())
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            String::from_utf8(start[5..7].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[8..10].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[11..13].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[14..16].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );

        NaiveDate::from_ymd(yr, mo, dy).and_hms(hr, mn, 0)
    }
}
