//! API for interacting with the database
//!
//! This module serves as the interface between the database and your app. It provides traits that
//! allow any implementing struct to perform CRUD operations.
use crate::form::Form;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;

use serde::Serialize;

/// Trait for creating a new database record
pub trait Creatable<E = DieselError> {
    type Output;
    /// Create new database record from an instance.
    fn create(&self, conn: &PgConnection) -> Result<Self::Output, E>;
}

/// Trait for retrieving records from database
pub trait Retrievable<T, Output = Self, Conn = PgConnection, E = DieselError>
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
pub trait Deletable<Output = Self, Conn = PgConnection, E = DieselError> {
    fn delete(&self, conn: &PgConnection) -> Result<Output, E>;
}

/// Trait for updating records
pub trait Updatable<Output = Self, E = DieselError> {
    /// Update the instance's corresponding db record. The updated struct is returned upon
    /// successful method call.
    fn update(&self) -> Result<Output, E>;
}
