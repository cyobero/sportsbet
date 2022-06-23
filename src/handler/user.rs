//! Module for proccessing HTTP requests
use super::DbPool;
use crate::form::{Auth, LoginForm};
use handlebars::Handlebars;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/signup")]
async fn signup_form(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> impl Responder {
    let body = hb.render("signup", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection.");
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
