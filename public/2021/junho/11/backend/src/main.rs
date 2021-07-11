pub mod database;
pub mod errors;
pub mod jwt;
mod router;

use log::LevelFilter;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rbatis;

#[launch]
async fn init() -> _ {
    log::set_max_level(LevelFilter::Info);

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()
    .expect("Falha ao definir CORS");

    database::init_database()
        .await
        .expect("Falha ao iniciar a Database");

    let rocket = rocket::build().attach(cors);

    router::get_routes(rocket)
}
