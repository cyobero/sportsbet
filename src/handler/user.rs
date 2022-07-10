//! Request handlers for user authentication
use super::DbPool;
use crate::db::Creatable;
use crate::form::{LoginForm, SignupForm};
use handlebars::Handlebars;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

/// Request handler for creating a new account from form data
#[post("/signup")]
async fn signup(
    pool: web::Data<DbPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<SignupForm>,
    _req: HttpRequest,
) -> impl Responder {
    web::block(move || {
        let conn = pool.get().expect("Could not establish connection.");
        form.0.validate().map(|f| {
            f.authenticate(&conn)
                .map(|nu| nu.create(&conn).map(|u| u.login(&conn)))
        })
    })
    .await
    .map(|f| match f {
        Ok(_) => {
            let body = hb
                .render(
                    "success",
                    &json!({"message": "successfuly created", "redirect": "/"}),
                )
                .unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            let body = hb
                .render("signup", &json!({"message": e.to_string() }))
                .unwrap();
            HttpResponse::Ok().body(body)
        }
    })
    .map_err(|e| {
        let body = hb
            .render("signup", &json!({"message": e.to_string() }))
            .unwrap();
        HttpResponse::Ok().body(body)
    })
}

/// Retrieve signup form
#[get("/signup")]
async fn signup_form(hb: web::Data<Handlebars<'_>>, _req: HttpRequest) -> impl Responder {
    let body = hb.render("signup", &{}).unwrap();
    HttpResponse::Ok().body(body)
}

/// Request handler for logging a user in
#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    form: web::Form<LoginForm>,
    hb: web::Data<Handlebars<'_>>,
) -> impl Responder {
    web::block(move || {
        let conn = pool.get().expect("Could not establish connection.");
        form.0.authenticate(&conn).map(|u| u.login(&conn))
    })
    .await
    .map(|_| {
        let body = hb
            .render(
                "success",
                &json!({"message": "login successful", "redirect": "/" }),
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

/// Retrieve login form
#[get("/login")]
async fn login_form(_req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let body = hb.render("login", &{}).unwrap();
    HttpResponse::Ok().body(body)
}
