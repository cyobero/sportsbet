//! Module for proccessing HTTP requests
use super::DbPool;
use crate::form::LoginForm;
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
    form: web::Form<LoginForm>,
    hb: web::Data<Handlebars<'_>>,
) -> impl Responder {
    let conn = pool.get().expect("Could not get connection");
    let res = form.authenticate(&conn).await;
    match res {
        Ok(_) => {
            let body = hb
                .render(
                    "success",
                    &json!({"message": "login successful", "redirect":"/events"}),
                )
                .unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            let body = hb
                .render("login", &json!({"message": e.to_string()}))
                .unwrap();
            HttpResponse::Ok().body(body)
        }
    }
}

#[get("/login")]
async fn login_form(_req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let body = hb.render("login", &{}).unwrap();
    HttpResponse::Ok().body(body)
}
