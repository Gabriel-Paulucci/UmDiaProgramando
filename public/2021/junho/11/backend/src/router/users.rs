use rbatis::{crud::CRUD, wrapper::Wrapper, DriverType};
use rocket::{http::Status, response::status, serde::json::Json};
use serde::Serialize;

use crate::database::{self, Usuario};

use super::auth::LoggedUser;

#[derive(Serialize)]
pub struct User {
    username: String,
}

#[get("/", format = "json")]
pub(super) async fn list(
    _logged: LoggedUser,
) -> Result<status::Accepted<Json<Vec<User>>>, status::Custom<&'static str>> {
    let wrapper = Wrapper::new(&DriverType::Sqlite);
    let data: Vec<Usuario> = database::DATABASE
        .fetch_list_by_wrapper(&wrapper)
        .await
        .map_err(|why| {
            log::error!("{:?}", why);
            status::Custom(Status::InternalServerError, "Fudeu")
        })?;

    let data = data
        .iter()
        .map(|e| User {
            username: e.username.clone(),
        })
        .collect::<Vec<_>>();

    Ok(status::Accepted(Some(Json(data))))
}
