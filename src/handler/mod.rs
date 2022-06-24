pub mod user;

use super::form::GameForm;
use super::model::{Event, Game, GameQuery, League, NewEvent, NewGame};
use super::DbPool;
use super::{NBA_TEAMS, NFL_TEAMS};
use crate::db::{Creatable, Retrievable};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

/// Request handler for getting all on-going games
#[get("/games")]
async fn get_games(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    query: web::Query<GameQuery>,
    _req: HttpRequest,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Game::query(&conn, &query.0))
        .await
        .map(|games| {
            let body = hb.render("games", &json!({ "games": games })).unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(move |e| {
            let body = hb
                .render("games", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
}

/// Request handler for posting a new Game from a form
#[post("/games/{league}/form")]
async fn post_game(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<GameForm>,
    path: web::Path<League>,
) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection.");
    let new = NewGame {
        league: path.0,
        home: form.home.to_owned(),
        away: form.away.to_owned(),
        start: form.start_to_naive(),
    };
    web::block(move || new.create(&conn))
        .await
        .map(|_| {
            let body = hb
                .render(
                    "success",
                    &json!({"message": "New game created", "redirect": "/games/form" }),
                )
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
#[get("/games/{league}/form")]
async fn games_form(
    hb: web::Data<Handlebars<'_>>,
    _req: HttpRequest,
    path: web::Path<League>,
) -> impl Responder {
    let teams: Vec<(&str, &str)> = match path.0 {
        League::NBA => NBA_TEAMS.to_vec(),
        League::NFL => NFL_TEAMS.to_vec(),
    };
    let body = hb.render("game_form", &json!({ "teams": teams })).unwrap();
    HttpResponse::Ok().body(body)
}

/// Request handler for retrieving all Events
#[get("/events")]
async fn get_events(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection");
    web::block(move || Event::all(&conn))
        .await
        .map(|evts| {
            let body = hb.render("events", &json!({ "events": evts })).unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(|e| {
            let body = hb
                .render("events", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
}

/// Request handler for getting a form for creating a new Event
#[get("/events/form")]
async fn event_form(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    _req: HttpRequest,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    web::block(move || Game::all(&conn))
        .await
        .map(|games| {
            let body = hb.render("event_form", &json!({ "games": games })).unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(|e| {
            let body = hb
                .render("event_form", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
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
                .render(
                    "success",
                    &json!({"message": "New Event created.", "redirect": "/events/form" }),
                )
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
