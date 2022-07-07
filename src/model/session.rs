use crate::db::*;
use crate::model::user::User;
use crate::schema::sessions::{self, dsl as sessions_dsl};
use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Queryable)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub login_date: NaiveDateTime,
    pub logout_date: Option<NaiveDateTime>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub user_id: i32,
    pub logout_date: Option<NaiveDateTime>,
}

impl Updatable for Session {
    fn update(&self, conn: &PgConnection) -> Result<Session, DieselError> {
        diesel::update(sessions_dsl::sessions)
            .filter(sessions_dsl::id.eq(&self.id))
            .set(sessions_dsl::logout_date.eq(&self.logout_date))
            .get_result(conn)
    }
}

impl Default for Session {
    fn default() -> Session {
        Session {
            id: -1,
            user_id: -1,
            login_date: Utc::now().naive_utc(),
            logout_date: None,
        }
    }
}

impl Default for NewSession {
    fn default() -> NewSession {
        NewSession {
            user_id: -1,
            logout_date: None,
        }
    }
}

impl NewSession {
    pub fn new(user: &User) -> Self {
        NewSession {
            user_id: user.id,
            logout_date: None,
        }
    }
}

impl Deletable for Session {
    fn delete(&self, conn: &PgConnection) -> Result<Session, DieselError> {
        diesel::delete(sessions_dsl::sessions)
            .filter(sessions_dsl::id.eq(&self.id))
            .get_result(conn)
    }
}

impl Creatable for NewSession {
    type Output = Session;
    fn create(&self, conn: &PgConnection) -> Result<Self::Output, DieselError> {
        diesel::insert_into(sessions_dsl::sessions)
            .values(self)
            .get_result(conn)
    }
}
