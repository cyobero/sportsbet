//! Module for proccessing HTTP requests
use super::DbPool;
use crate::db::{Creatable, Retrievable};
use crate::form::{Auth, GameForm, LoginForm};
use crate::model::user::{AuthedUser, User};
use crate::model::{Event, Game, NewEvent, NewGame};
use crate::schema::users::{self, dsl as users_dsl};
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use handlebars::Handlebars;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
    let pass = form.clone().password;
    web::block(move || form.0.validate(&conn))
        .await
        .map(|_| {
            let body = hb
                .render(
                    "success",
                    &json!({"message": "login succesful", "redirect": "/events "}),
                )
                .unwrap();
            HttpResponse::Ok().body(body)
        })
        .map_err(|e| {
            let body = hb
                .render("login", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::Ok().body(body)
        })
}

#[get("/login")]
async fn login_form(_req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let body = hb.render("login", &{}).unwrap();
    HttpResponse::Ok().body(body)
}
