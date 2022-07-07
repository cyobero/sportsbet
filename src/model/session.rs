use crate::db::Creatable;
use crate::schema::sessions::{self, dsl as sessions_dsl};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{Insertable, Queryable, RunQueryDsl};
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

impl Creatable for NewSession {
    type Output = Session;
    fn create(&self, conn: &PgConnection) -> Result<Self::Output, DieselError> {
        diesel::insert_into(sessions_dsl::sessions)
            .values(self)
            .get_result(conn)
    }
}
