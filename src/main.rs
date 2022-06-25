#[macro_use]
extern crate diesel;

pub mod db;
pub mod form;
pub mod handler;
pub mod model;
pub mod schema;
pub mod test;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use handlebars::Handlebars;
use handler::*;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod exports {
    pub use crate::model::user::RoleMapping as Role;
    pub use crate::model::LeagueMapping as League;
}
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

static NFL_TEAMS: [(&'static str, &'static str); 27] = [
    ("ATL", "Atlanta Falcons"),
    ("ARI", "Arizona Cardinals"),
    ("BAL", "Baltimore Ravens"),
    ("BUF", "Buffalo Bills"),
    ("CHI", "Chicago Bears"),
    ("CIN", "Cincinnati Bengals"),
    ("CLE", "Cleveland Browns"),
    ("DAL", "Dallas Cowboys"),
    ("DEN", "Denver Broncos"),
    ("DET", "Detroit Lions"),
    ("GB", "Green Bay Packers"),
    ("HOU", "Houston Texans"),
    ("IND", "Indianapolis Colts"),
    ("JAX", "Jacksonville Jaguars"),
    ("KC", "Kansas City Chiefs"),
    ("LAC", "Los Angeles Chargers"),
    ("LAR", "Los Angeles Rams"),
    ("MIA", "Miami Dolphins"),
    ("LV", "Las Vegas Raiders"),
    ("NE", "New England Patriots"),
    ("NYG", "New York Giants"),
    ("NYJ", "New York Jets"),
    ("PIT", "Pittsburgh Steelers"),
    ("SEA", "Seattle Seahawks"),
    ("SF", "San Francisco 49ers"),
    ("TEN", "Tennessee Titans"),
    ("WAS", "Washington Commanders"),
];

static NBA_TEAMS: [(&'static str, &'static str); 30] = [
    ("ATL", "Atlanta Hawks"),
    ("BOS", "Boston Celtics"),
    ("BKN", "Brooklyn Nets"),
    ("CHA", "Charlotte Hornets"),
    ("CHI", "Chicago Bulls"),
    ("CLE", "Cleveland Cavaliers"),
    ("DAL", "Dallas Mavericks"),
    ("DEN", "Denver Nuggets"),
    ("DET", "Detroit Pistons"),
    ("GSW", "Golden State Warriors"),
    ("HOU", "Houston Rockets"),
    ("IND", "Indiana Pacers"),
    ("LAC", "Los Angeles Clippers"),
    ("LAL", "Los Angeles Lakers"),
    ("MEM", "Memphis Grizzlies"),
    ("MIA", "Miami Heat"),
    ("MIL", "Milwaukee Bucks"),
    ("MIN", "Minnesota Timberwolves"),
    ("NOP", "New Orleans Pelicans"),
    ("NYK", "New York Knicks"),
    ("OKC", "Oklahoma City Thunder"),
    ("ORL", "Orlando Magic"),
    ("PHI", "Philadelphia 76ers"),
    ("PHX", "Phoenix Suns"),
    ("POR", "Portland Trailblazers"),
    ("SAC", "Sacramento Kings"),
    ("SAS", "San Antonio Spurs"),
    ("TOR", "Toronto Raptors"),
    ("UTA", "Utah Jazz"),
    ("WAS", "Washington Wizards"),
];

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    println!(
        "dtabase url: {}",
        env::var("DATABASE_URL").expect("gang shit")
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let styles = r#"
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css">
    "#;
    handlebars.register_partial("styles", styles).unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let addrress = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8305);
    println!("🚀 ⛽🌬️🌬️ Serving at {:?}", addrress);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(Files::new("/static", "./static"))
            .service(index)
            .service(get_events)
            .service(event_form)
            .service(post_event)
            .service(games_form)
            .service(post_game)
            .service(get_games)
            .service(user::login_form)
            .service(user::login)
    })
    .bind(addrress)?
    .run()
    .await
}
