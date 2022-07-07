use crate::db::{Creatable, Deletable, Retrievable};
use crate::form::LoginForm;
use crate::schema::users::{self, dsl as users_dsl};

use diesel::pg::PgConnection;
use diesel::sql_types::{Integer, Varchar};
use diesel::{sql_query, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
type DieselError = diesel::result::Error;

pub trait Form {}

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
    pub password: String,
    #[sql_type = "RoleMapping"]
    pub role: Role,
}

#[derive(Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "users"]
pub struct AuthedUser {
    #[sql_type = "Varchar"]
    pub email: String,
    #[sql_type = "Varchar"]
    pub username: String,
    #[sql_type = "Varchar"]
    pub password: String,
    #[sql_type = "RoleMapping"]
    pub role: Role,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserQuery<'a> {
    pub email: &'a str,
    pub username: &'a str,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                               //
/////// Implementations ///////////////////////////////////////////////////////////////////////////
//                                                                                               //
///////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for User {
    fn default() -> User {
        User {
            id: -1,
            email: String::new(),
            username: String::new(),
            password: String::new(),
            role: Role::Punter,
        }
    }
}

impl Deletable for User {
    fn delete(&self, conn: &PgConnection) -> Result<User, DieselError> {
        diesel::delete(users_dsl::users.filter(users_dsl::email.eq(&self.email))).get_result(conn)
    }
}

impl Retrievable<UserQuery<'_>> for User {
    fn query(conn: &PgConnection, data: &UserQuery) -> Result<Vec<User>, DieselError> {
        let stmt = format!(
            "SELECT * FROM users WHERE email = '{}' or username ='{}'",
            data.email, data.username
        );
        sql_query(stmt).load(conn)
    }

    fn all(conn: &PgConnection) -> Result<Vec<User>, DieselError> {
        users_dsl::users.load(conn)
    }
}

impl Creatable for NewUser {
    type Output = User;
    fn create(&self, conn: &PgConnection) -> Result<User, DieselError> {
        diesel::insert_into(users_dsl::users)
            .values(self)
            .get_result(conn)
    }
}
