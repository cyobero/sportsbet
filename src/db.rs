//! API for interacting with the database
//!
//! This module serves as the interface between the database and your app. It provides traits that
//! allow any implementing struct to perform CRUD operations.

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
    /// Retrieve a database object by passing in some data `T` to query. `data` can be anything that
    /// implements `Serialize`. This queries the database based on the fields and values of the
    /// passed-in `data` struct reference.
    fn query(conn: &Conn, data: &T) -> Result<Vec<Output>, E>;

    /// Retrieves all records from database.
    fn all(conn: &Conn) -> Result<Vec<Output>, E>;
}

/// Trait for deleting records
pub trait Deletable<E = result::Error> {
    fn delete(&self) -> Result<(), E>;
}

/// Trait for updating records
pub trait Updatable<Output = Self, E = result::Error> {
    /// Update the instance's corresponding db record. The updated struct is returned upon
    /// successful method call.
    fn update(&self) -> Result<Output, E>;
}
