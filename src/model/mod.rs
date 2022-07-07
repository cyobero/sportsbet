pub mod session;
pub mod user;

use super::db::{Creatable, Deletable, Retrievable};
use super::schema::events::{self, dsl as events_dsl};
use super::schema::games::{self, dsl as games_dsl};

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::sql_types::{Integer, Timestamp, Varchar};
use diesel::{sql_query, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, DbEnum, Deserialize, Serialize, PartialEq)]
pub enum League {
    NBA,
    NFL,
}

#[derive(Clone, Debug, Deserialize, Serialize, Queryable, QueryableByName)]
pub struct Game {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "LeagueMapping"]
    pub league: League,
    #[sql_type = "Varchar"]
    pub home: String,
    #[sql_type = "Varchar"]
    pub away: String,
    #[sql_type = "Timestamp"]
    pub start: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub league: League,
    pub home: String,
    pub away: String,
    pub start: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Event {
    pub id: i32,
    pub description: String,
    pub odds: i32,
    pub game_id: Option<i32>,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub game_id: i32,
    pub description: String,
    pub odds: i32,
}

#[derive(Clone, Copy, Serialize)]
pub struct EventQuery {
    pub id: Option<i32>,
    pub odds: Option<i32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GameQuery {
    pub league: Option<League>,
}

impl ToString for League {
    fn to_string(&self) -> String {
        match self {
            League::NBA => "NBA".to_string(),
            League::NFL => "NFL".to_string(),
        }
    }
}

impl Default for GameQuery {
    fn default() -> Self {
        GameQuery { league: None }
    }
}

impl Retrievable<GameQuery> for Game {
    fn query(conn: &PgConnection, q: &GameQuery) -> Result<Vec<Game>, DieselError> {
        let stmt = match q.league {
            Some(League::NBA) => "SELECT * FROM games WHERE league = 'nba' and id NOT IN (SELECT game_id FROM game_results)",
            Some(League::NFL) => "SELECT * FROM games WHERE league = 'nfl' and id NOT IN (SELECT game_id FROM game_results)",
            None => "SELECT * FROM games WHERE id NOT IN (SELECT game_id FROM game_results)"
        };
        sql_query(stmt).get_results(conn)
    }

    /// Retrieves all games that don't have a result (i.e. don't have a final score)
    fn all(conn: &PgConnection) -> Result<Vec<Game>, DieselError> {
        let _stmt = "SELECT * FROM games WHERE games.id NOT IN (SELECT game_id FROM game_results)";
        sql_query(_stmt).load(conn)
    }
}

impl Creatable for NewGame {
    type Output = Game;
    fn create(&self, conn: &PgConnection) -> Result<Game, DieselError> {
        diesel::insert_into(games_dsl::games)
            .values(self)
            .get_result(conn)
    }
}

impl Deletable for Event {
    fn delete(&self, conn: &PgConnection) -> Result<Event, DieselError> {
        diesel::delete(events_dsl::events.filter(events_dsl::id.eq(&self.id))).get_result(conn)
    }
}

impl Retrievable<EventQuery> for Event {
    fn query(conn: &PgConnection, data: &EventQuery) -> Result<Vec<Event>, DieselError> {
        match data {
            EventQuery {
                id: Some(_id),
                odds: None,
            } => events_dsl::events
                .filter(events_dsl::id.eq(_id))
                .get_results(conn),

            EventQuery {
                id: None,
                odds: Some(_odds),
            } => events_dsl::events
                .filter(events_dsl::odds.eq(_odds))
                .get_results(conn),

            EventQuery {
                id: Some(_id),
                odds: Some(_odds),
            } => events_dsl::events
                .filter(events_dsl::id.eq(_id))
                .filter(events_dsl::odds.eq(_odds))
                .get_results(conn),

            EventQuery {
                id: None,
                odds: None,
            } => events_dsl::events.load(conn),
        }
    }

    fn all(conn: &PgConnection) -> Result<Vec<Event>, DieselError> {
        events_dsl::events
            .limit(100)
            .order_by(events_dsl::timestamp.desc())
            .load(conn)
    }
}

impl Creatable for NewEvent {
    type Output = Event;
    fn create(&self, conn: &PgConnection) -> Result<Event, DieselError> {
        diesel::insert_into(events_dsl::events)
            .values(self)
            .get_result(conn)
    }
}
