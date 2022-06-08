#[macro_use]
extern crate diesel;

pub mod db;
pub mod handler;
pub mod model;
pub mod schema;
pub mod test;

use handlebars::Handlebars;
use handler::*;

use actix_web::{web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let addrress = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8305);
    println!("🚀 ⛽🌬️🌬️ Serving at {:?}", addrress);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(get_events)
            .service(event_form)
            .service(post_event)
    })
    .bind(addrress)?
    .run()
    .await
}
