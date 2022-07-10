//! Module for form handling

use std::fmt;

use crate::db::Retrievable;
use crate::model::user::Role;
use crate::model::user::{AuthedUser, NewUser, User, UserQuery};
use crate::schema::events;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
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

#[derive(Debug, Clone)]
pub enum ValidationError {
    PasswordMismatch,
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
pub struct SignupForm {
    pub email: String,
    pub username: String,
    pub password1: String,
    pub password2: String,
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
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::PasswordMismatch => write!(f, "PasswordMismatch"),
        }
    }
}
impl Form for SignupForm {}
impl Form for LoginForm {}
impl error::Error for AuthError {}
impl error::Error for ValidationError {}

impl SignupForm {
    pub fn new() -> Self {
        SignupForm {
            email: String::new(),
            username: String::new(),
            password1: String::new(),
            password2: String::new(),
            role: Role::Punter,
        }
    }

    /// Validates form by checking if passwords match
    pub fn validate(self) -> Result<Self, impl error::Error> {
        if &self.password2 == &self.password1 {
            Ok(self)
        } else {
            Err(ValidationError::PasswordMismatch)
        }
    }

    /// Authenticates signup form by checking database to see if email or username is available
    pub fn authenticate(self, conn: &PgConnection) -> Result<NewUser, AuthError> {
        let usr = User::query(
            conn,
            &UserQuery {
                email: &self.email,
                username: &self.username,
            },
        )
        .unwrap();
        if usr.len() > 0 {
            Err(AuthError::EmailTaken)
        } else {
            Ok(NewUser {
                email: self.email,
                username: self.username,
                password: self.password2,
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
    pub fn authenticate(self, conn: &PgConnection) -> Result<User, AuthError> {
        let usrs = User::query(
            &conn,
            &UserQuery {
                email: &self.email,
                username: "",
            },
        )
        .unwrap();
        match usrs.len() {
            0 => Err(AuthError::EmailNotFound),
            _ => match &usrs[0].password == &self.password {
                true => Ok(usrs[0].clone()),
                false => Err(AuthError::IncorrectPassword),
            },
        }
    }

    /// Return the associated user object or None if no user is found
    pub async fn user(&self, conn: &PgConnection) -> Option<User> {
        let usrs = User::query(
            conn,
            &UserQuery {
                email: &self.email,
                username: "",
            },
        )
        .unwrap();
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
