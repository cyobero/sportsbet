#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    let addrress = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8305);
    println!("🚀 ⛽🌬️🌬️ Serving at {:?}", addrress);
}
