mod auth;
mod users;

use rocket::{Build, Rocket};

pub fn get_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/auth", routes![auth::login])
        .mount("/users", routes![users::list])
}
