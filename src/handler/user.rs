//! Module for proccessing HTTP requests
use super::DbPool;
use crate::db::{Creatable, Retrievable};
use crate::form::GameForm;
use crate::model::{Event, Game, NewEvent, NewGame};
use handlebars::Handlebars;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/login")]
async fn login(_req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let body = hb.render("login", &{}).unwrap();
    HttpResponse::Ok().body(body)
}
