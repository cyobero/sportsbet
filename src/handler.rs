//! Module for proccessing HTTP requests
use super::db::{Creatable, Retrievable};
use super::model::{Event, NewEvent};
use super::DbPool;

use super::NBA_TEAMS;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

/// Request handler for retrieving the form to create a new Game
#[get("/games/form")]
async fn games_form(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> impl Responder {
    let body = hb
        .render("games_form", &json!({ "teams": NBA_TEAMS }))
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
