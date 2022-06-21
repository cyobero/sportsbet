use crate::db::Retrievable;
use crate::model::user::{AuthedUser, User};
use crate::schema::events;
use actix_web::web::Form;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum AuthError {
    EmailNotFound,
}

pub trait Auth<C = PgConnection, E = AuthError> {
    type Output;
    fn authenticate(self, conn: &C) -> Result<Self::Output, E>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

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

impl Auth for LoginForm {
    type Output = AuthedUser;
    fn authenticate(self, conn: &PgConnection) -> Result<AuthedUser, AuthError> {
        let res = User::query(conn, &self);
        match res {
            Ok(usrs) => match usrs.len() {
                0 => Err(AuthError::EmailNotFound),
                _ => Ok(usrs[0].clone()),
            },
            Err(e) => Err(AuthError::EmailNotFound),
        }
    }
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
