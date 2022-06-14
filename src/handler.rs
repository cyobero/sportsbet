//! Module for proccessing HTTP requests
use super::db::{Creatable, Retrievable};
use super::model::{Event, NewEvent, NewGame};
use super::schema::games::dsl::games;
use super::DbPool;

use super::NBA_TEAMS;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::result::Error as DieselError;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use substring::Substring;

#[derive(Debug, Deserialize, Serialize)]
struct GameForm {
    home: String,
    away: String,
    start: String,
}

impl GameForm {
    pub fn new() -> Self {
        GameForm {
            home: "HOME".to_owned(),
            away: "AWAY".to_owned(),
            start: "1987-10-03T17:00:00".to_owned(),
        }
    }

    // Utility function to convert `start` and return a NaiveDateTime type instead of a String
    pub fn start_to_naive(&self) -> NaiveDateTime {
        let start = self.start.as_bytes();
        let (yr, mo, dy, hr, mn) = (
            String::from_utf8(start[0..4].to_vec())
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            String::from_utf8(start[5..7].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[8..10].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[11..13].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            String::from_utf8(start[14..16].to_vec())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );

        NaiveDate::from_ymd(yr, mo, dy).and_hms(hr, mn, 0)
    }
}

/// Request handler for posting a new Game from a form
#[post("/games/form")]
async fn post_game(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<GameForm>,
) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection.");
    let new = NewGame {
        home: form.home.to_owned(),
        away: form.away.to_owned(),
        start: form.start_to_naive(),
    };
    web::block(move || new.create(&conn))
        .await
        .map(|_| {
            let body = hb
                .render("success", &json!({"message": "New game created"}))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(|e| {
            let body = hb
                .render("game_form", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::InternalServerError().body(body)
        })
}

/// Request handler for retrieving the form to create a new Game
#[get("/games/form")]
async fn games_form(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> impl Responder {
    let body = hb
        .render(
            "game_form",
            &json!({ "teams": NBA_TEAMS, "dt": Local::now().naive_utc() }),
        )
        .unwrap();
    HttpResponse::Ok().body(body)
}

/// Request handler for retrieving all Events
#[get("/v1/events")]
async fn get_events(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection");
    web::block(move || Event::all(&conn))
        .await
        .map(|dta| web::Json(dta))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(json!({"status": 404, "data": e.to_string() }))
        })
}

/// Request handler for getting a form for creating a new Event
#[get("/events/form")]
async fn event_form(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> impl Responder {
    let body = hb.render("event_form", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

/// Request handler for posting event forms.
#[post("/events/form")]
async fn post_event(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<NewEvent>,
    _req: HttpRequest,
) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection.");
    web::block(move || form.0.create(&conn))
        .await
        .map(|_| {
            let body = hb
                .render("event_form", &json!({"message": "New Event created."}))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(|e| {
            let body = hb
                .render("event_form", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::InternalServerError().body(body)
        })
}
