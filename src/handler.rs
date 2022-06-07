//! Module for proccessing HTTP requests
use super::db::Retrievable;
use super::model::Event;
use super::DbPool;

use actix_web::{get, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Request handler for retrieving all Events
#[get("/v1/events")]
async fn get_events(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection");
    web::block(move || Event::all(&conn))
        .await
        .map(|res| HttpResponse::Ok().json(json!({"status": 200, "data": res })))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(json!({"status": 404, "data": e.to_string() }))
        })
}
