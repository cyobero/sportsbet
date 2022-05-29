//! An API for interacting with Postgres

use diesel::pg::PgConnection;
use diesel::result;

use serde::Serialize;

/// Trait for creating a new database record
pub trait Creatable<E = result::Error> {
    type Output;
    /// Create new database record from an instance.
    fn create(&self, conn: &PgConnection) -> Result<Self::Output, E>;
}

/// Trait for retrieving records from database
pub trait Retrievable<T, Output = Self, Conn = PgConnection, E = result::Error>
where
    T: Serialize,
{
    type Output;
    /// Retrieve a database object by passing in some data `T` to query. `data` can be anything that
    /// implements `Serialize`. This queries the database based on the fields and values of the
    /// passed-in `data` struct.
    fn query(conn: &Conn, data: T) -> Result<Vec<Self::Output>, E>;

    /// Retrieves all records from database.
    fn all(conn: &Conn) -> Result<Vec<Self::Output>, E>;
}
