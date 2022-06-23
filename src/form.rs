//! A module for form handling

use crate::db::Retrievable;
use crate::model::user::Role;
use crate::model::user::{AuthedUser, User};
use crate::schema::events;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::Insertable;
use diesel::{sql_query, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum AuthError {
    EmailNotFound,
    IncorrectPassword,
    Taken,
}

pub trait Auth<C = PgConnection, E = AuthError> {
    type Output;
    fn authenticate(&self, conn: &C) -> Result<Self::Output, E>;
    fn validate(&self, conn: &C) -> Result<Self::Output, E>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignupForm<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password1: &'a str,
    pub password2: &'a str,
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

///////////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                               //
/////// Implementations ///////////////////////////////////////////////////////////////////////////
//                                                                                               //
///////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for AuthedUser {
    fn default() -> Self {
        AuthedUser {
            email: String::new(),
            username: String::new(),
            password: String::new(),
            role: Role::Punter,
        }
    }
}

impl Auth for SignupForm<'_> {
    type Output = AuthedUser;
    fn authenticate(&self, conn: &PgConnection) -> Result<AuthedUser, AuthError> {
        let _stmt = format!(
            "SELECT email, username, role FROM users WHERE email = '{}' OR username = '{}'",
            &self.email, &self.username
        );
        let res = sql_query(_stmt).get_results::<AuthedUser>(conn);
        match res {
            Ok(_) => Err(AuthError::Taken),
            Err(_) => Ok(AuthedUser::default()),
        }
    }

    fn validate(&self, conn: &PgConnection) -> Result<AuthedUser, AuthError> {
        unimplemented!()
    }
}

impl Auth for LoginForm {
    type Output = User;
    fn authenticate(&self, conn: &PgConnection) -> Result<User, AuthError> {
        let res = User::query(conn, &self);
        match res {
            Ok(usrs) => match usrs.len() {
                0 => Err(AuthError::EmailNotFound),
                _ => Ok(usrs[0].clone()),
            },
            Err(_) => Err(AuthError::EmailNotFound),
        }
    }

    fn validate(&self, conn: &PgConnection) -> Result<User, AuthError> {
        let usr = self.authenticate(conn);
        match usr {
            Ok(u) => {
                if &u.password == &self.password {
                    Ok(u)
                } else {
                    Err(AuthError::IncorrectPassword)
                }
            }
            Err(e) => Err(e),
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
