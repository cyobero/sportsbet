use crate::db::Creatable;
use crate::schema::users::{self, dsl as users_dsl};

use diesel::pg::PgConnection;
use diesel::sql_types::{Integer, Varchar};
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
type DieselError = diesel::result::Error;

#[derive(Clone, Copy, Debug, DbEnum, Deserialize, Serialize, PartialEq)]
pub enum Role {
    Bookie,
    Punter,
}

#[derive(Clone, Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Clone, Deserialize, Serialize, Queryable, QueryableByName)]
#[table_name = "users"]
pub struct User {
    #[sql_type = "Integer"]
    id: i32,
    #[sql_type = "Varchar"]
    pub email: String,
    #[sql_type = "Varchar"]
    pub username: String,
    #[sql_type = "Varchar"]
    password: String,
    #[sql_type = "RoleMapping"]
    pub role: Role,
}

impl Creatable for NewUser {
    type Output = User;
    fn create(&self, conn: &PgConnection) -> Result<User, DieselError> {
        diesel::insert_into(users_dsl::users)
            .values(self)
            .get_result(conn)
    }
}
