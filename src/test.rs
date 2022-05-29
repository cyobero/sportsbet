use super::db::*;
use super::model::*;

use diesel::pg::PgConnection;
use diesel::{Connection, ConnectionError};

use dotenv::dotenv;

use std::env;

fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

fn setup_environment() {
    dotenv().ok();
    let conn = establish_connection().unwrap();
    let ne = NewEvent {
        description: "CHI vs OKC (O 218.5)".to_owned(),
        odds: -110,
    };
    let event = ne.create(&conn);
}

#[cfg(test)]
mod db_tests {
    use super::{establish_connection, setup_environment};
    use crate::db::*;
    use crate::model::*;
    use crate::schema::events::{self, dsl};
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    #[test]
    fn new_event_created_is_ok() {
        let conn = establish_connection().unwrap();
        let new = NewEvent {
            description: "CHI (+3) vs DET (-3)".to_owned(),
            odds: -110,
        };
        let event = new.create(&conn).unwrap();
        assert_eq!(event.odds, -110);
        assert_eq!(event.description, "CHI (+3) vs DET (-3)".to_owned());
        let _ = diesel::delete(dsl::events.filter(dsl::id.eq(event.id))).get_result::<Event>(&conn);
    }

    #[test]
    fn all_events_retrieved() {
        setup_environment();
        let conn = establish_connection().unwrap();
        let all = Event::all(&conn).unwrap();
        assert_eq!(all.is_empty(), false);
    }
}
