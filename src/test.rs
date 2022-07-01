use diesel::pg::PgConnection;
use diesel::{Connection, ConnectionError};

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
}

#[cfg(test)]
mod form_tests {
    use super::establish_connection;
    use crate::form::*;

    #[test]
    fn signup_email_available() {
        let conn = establish_connection().unwrap();
        let dta = SignupForm {
            email: "available@email.com".to_owned(),
            username: "foobars".to_owned(),
            password1: "password".to_owned(),
            password2: "password".to_owned(),
            role: crate::model::user::Role::Punter,
        };
    }

    #[actix_web::main]
    #[test]
    async fn signup_email_taken() {
        let conn = establish_connection().unwrap();
        let dta = SignupForm {
            email: "foo@bar.com".to_owned(),
            username: "foobars".to_owned(),
            password1: "password".to_owned(),
            password2: "password".to_owned(),
            role: crate::model::user::Role::Bookie,
        };
        let res = dta.authenticate(&conn);
        assert!(res.is_err())
    }

    #[actix_web::main]
    #[test]
    async fn associated_user_returned() {
        let conn = establish_connection().unwrap();
        let form = LoginForm {
            email: "foo@bar.com".to_owned(),
            password: "password".to_owned(),
        };
        let usr = form.user(&conn).await.unwrap();
        assert_eq!(usr.email, "foo@bar.com".to_owned());
    }

    #[actix_web::main]
    #[test]
    async fn no_associated_user_returned() {
        let conn = establish_connection().unwrap();
        let form = LoginForm {
            email: "doesnt@exist.com".to_owned(),
            password: "password".to_owned(),
        };
        let usr = form.user(&conn).await;
        assert!(usr.is_none());
    }

    #[test]
    fn user_authenticated() {
        let conn = establish_connection().unwrap();
        let form = LoginForm {
            email: "foo@bar.com".to_string(),
            password: "password".to_string(),
        };
        let usr = form.authenticate(&conn);
        assert!(usr.is_ok())
    }

    #[test]
    fn password_validated() {
        use crate::model::user::Role;
        let form = SignupForm {
            email: "cyobero@gmail.com".to_string(),
            username: "cyobero".to_string(),
            password1: "password123".to_string(),
            password2: "password123".to_string(),
            role: Role::Bookie,
        };
        let res = form.validate();
        assert!(res.is_ok());
    }
}

#[cfg(test)]
mod db_tests {
    use super::establish_connection;
    use crate::db::*;
    use crate::model::*;
    use crate::schema::events::{self, dsl};
    use chrono::naive::{NaiveDate, NaiveDateTime};
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    #[test]
    fn new_event_created_is_ok() {
        let conn = establish_connection().unwrap();
        let new = NewEvent {
            description: "CHI (+3) vs DET (-3)".to_owned(),
            game_id: 1,
            odds: -110,
        };
        let event = new.create(&conn).unwrap();
        assert_eq!(event.odds, -110);
        assert_eq!(event.description, "CHI (+3) vs DET (-3)".to_owned());
        let _ = diesel::delete(dsl::events.filter(dsl::id.eq(event.id))).get_result::<Event>(&conn);
    }

    #[test]
    fn all_events_retrieved() {
        let conn = establish_connection().unwrap();
        let all = Event::all(&conn).unwrap();
        assert_eq!(all.is_empty(), false);
    }

    #[test]
    fn query_retrieves_correct_results() {
        let conn = establish_connection().unwrap();
        let data = vec![
            NewEvent {
                description: "FOO vs BAR (-6.5)".to_owned(),
                game_id: 1,
                odds: -105,
            },
            NewEvent {
                description: "FOO (+6.5) vs BAR".to_owned(),
                game_id: 1,
                odds: -110,
            },
            NewEvent {
                description: "CHI vs BOS U 51.5".to_owned(),
                game_id: 1,
                odds: -110,
            },
        ];

        for ne in data {
            ne.create(&conn).unwrap();
        }

        let res = Event::query(
            &conn,
            &EventQuery {
                id: None,
                odds: Some(-110),
            },
        )
        .unwrap();

        for r in res {
            assert_eq!(r.odds, -110);
            let _ = r.delete(&conn);
        }
    }

    #[test]
    fn query_returns_zero_results() {
        let conn = establish_connection().unwrap();
        let res = Event::query(
            &conn,
            &EventQuery {
                id: None,
                odds: Some(1_000_000),
            },
        );
        assert_eq!(res.unwrap().len(), 0);
    }

    #[test]
    fn record_deleted() {
        let conn = establish_connection().unwrap();
        let new = NewEvent {
            description: "test".to_string(),
            game_id: 1,
            odds: 110,
        };
        let event = new.create(&conn).unwrap();
        let deleted = event.delete(&conn);
        assert!(deleted.is_ok());
        let res = Event::query(
            &conn,
            &EventQuery {
                id: Some(event.id),
                odds: None,
            },
        )
        .unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn game_created() {
        use crate::schema::games::dsl::*;
        let conn = establish_connection().unwrap();
        let new = NewGame {
            league: League::NBA,
            home: "BOS".to_string(),
            away: "GSW".to_string(),
            start: NaiveDate::from_ymd(2022, 06, 08).and_hms(17, 30, 0),
        };
        let game = new.create(&conn).unwrap();
        assert_eq!(game.away, "GSW".to_string());
        let _ = diesel::delete(games.find(game.id)).get_result::<Game>(&conn);
    }

    #[test]
    fn all_games_retrieved() {
        let conn = establish_connection().unwrap();
        let games = Game::all(&conn).unwrap();
        assert_ne!(games.len(), 0);
    }

    #[test]
    fn user_created_and_deleted() {
        use crate::model::user::NewUser;
        use crate::model::user::Role::*;
        let usr = NewUser {
            email: "foo1@bar.com".to_string(),
            username: "test-user".to_string(),
            password: "password".to_string(),
            role: Bookie,
        };
        let conn = establish_connection().unwrap();
        let new = usr.create(&conn).unwrap();
        assert_eq!(new.role, Bookie);
        let del = new.delete(&conn);
        assert!(del.is_ok());
    }

    #[test]
    fn user_queried() {
        use crate::model::user::{User, UserQuery};
        let conn = establish_connection().unwrap();
        let res = User::query(
            &conn,
            &UserQuery {
                email: "foo@bar.com",
            },
        )
        .unwrap();
        assert_ne!(res.len(), 0);
    }

    #[test]
    fn nba_games_retrieved() {
        let conn = establish_connection().unwrap();

        let games = Game::query(
            &conn,
            &GameQuery {
                league: Some(League::NBA),
            },
        )
        .unwrap();
        assert_ne!(games.len(), 0);
        assert_eq!(games[0].league, League::NBA);
    }

    #[test]
    fn nfl_games_retrieved() {
        let conn = establish_connection().unwrap();

        let games = Game::query(
            &conn,
            &GameQuery {
                league: Some(League::NFL),
            },
        )
        .unwrap();
        assert_ne!(games.len(), 0);
        assert_eq!(games[0].league, League::NFL);
    }

    #[test]
    fn all_games_retrieved_no_league_input() {
        let conn = establish_connection().unwrap();
        let games = Game::query(&conn, &GameQuery { league: None }).unwrap();
        assert_ne!(games.len(), 0);
    }
}
