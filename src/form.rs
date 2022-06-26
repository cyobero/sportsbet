//! A module for form handling

use std::fmt;

use crate::db::Retrievable;
use crate::model::user::Role;
use crate::model::user::{AuthedUser, NewUser, User, UserQuery};
use crate::schema::events;
use actix_web::web;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::backend::Backend;
use diesel::pg::PgConnection;
use diesel::{sql_query, QueryDsl, Queryable, RunQueryDsl};
use diesel::{Connection, Insertable};
use serde::{Deserialize, Serialize};
use std::error;

pub trait Form {}

#[derive(Debug, Clone)]
pub enum AuthError {
    EmailNotFound,
    EmailTaken,
    IncorrectPassword,
}

#[async_trait]
pub trait Auth<C: Connection, E = AuthError>
where
    E: error::Error,
{
    type Output;
    async fn authenticate(&self, conn: &C) -> Result<Self::Output, E>;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                               //
/////// Structs ///////////////////////////////////////////////////////////////////////////////////
//                                                                                               //
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignupForm<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password1: &'a str,
    pub password2: &'a str,
    pub role: Role,
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
impl Form for SignupForm<'_> {}
impl Form for LoginForm {}
impl error::Error for AuthError {}

impl SignupForm<'_> {
    pub fn new() -> Self {
        SignupForm {
            email: "",
            username: "",
            password1: "",
            password2: "",
            role: Role::Punter,
        }
    }

    pub async fn authenticate(&self, conn: &PgConnection) -> Result<NewUser, AuthError> {
        let usr = User::query(conn, &UserQuery { email: self.email }).unwrap();
        if usr.len() > 0 {
            Err(AuthError::EmailTaken)
        } else {
            Ok(NewUser {
                email: self.email.to_owned(),
                username: self.username.to_owned(),
                password: self.password2.to_owned(),
                role: self.role,
            })
        }
    }
}

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

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::IncorrectPassword => write!(f, "IncorrectPassword"),
            AuthError::EmailNotFound => write!(f, "EmailNotFound"),
            AuthError::EmailTaken => write!(f, "EmailTaken"),
        }
    }
}

impl LoginForm {
    pub fn new() -> Self {
        LoginForm {
            email: String::new(),
            password: String::new(),
        }
    }

    /// Check the form instance's password against the associated user object's password
    pub async fn authenticate(&self, conn: &PgConnection) -> Result<User, AuthError> {
        match self.user(conn).await {
            None => Err(AuthError::EmailNotFound),
            Some(u) => {
                if self.password == u.password {
                    Ok(u)
                } else {
                    Err(AuthError::IncorrectPassword)
                }
            }
        }
    }

    /// Return the associated user object or None if no user is found
    pub async fn user(&self, conn: &PgConnection) -> Option<User> {
        let usrs = User::query(conn, &UserQuery { email: &self.email }).unwrap();
        if usrs.len() > 0 {
            Some(usrs[0].clone())
        } else {
            None
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
