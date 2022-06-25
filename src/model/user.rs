use crate::db::{Creatable, Deletable, Retrievable};
use crate::form::LoginForm;
use crate::schema::users::{self, dsl as users_dsl};

use diesel::pg::PgConnection;
use diesel::sql_types::{Integer, Varchar};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
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

impl Retrievable<LoginForm> for User {
    fn query(conn: &PgConnection, data: &LoginForm) -> Result<Vec<User>, DieselError> {
        users_dsl::users
            .filter(users_dsl::email.eq(&data.email))
            .get_results(conn)
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
