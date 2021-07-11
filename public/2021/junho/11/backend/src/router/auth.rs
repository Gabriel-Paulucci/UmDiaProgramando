use std::collections::BTreeMap;

use rbatis::crud::CRUD;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status,
    serde::json::Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::{self, Usuario},
    errors::{AuthError, DatabaseError, Errors},
    jwt::{self, JwtConvetClaim},
};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenData {
    username: String,
    token: String,
}

struct Tokenzin;

impl JwtConvetClaim for Tokenzin {
    fn get_values_from_clain(
        _: &std::collections::BTreeMap<String, String>,
    ) -> Result<Box<Self>, crate::errors::AuthError> {
        Ok(Self.into())
    }

    fn set_clains(&self) -> std::collections::BTreeMap<&'static str, String> {
        BTreeMap::new()
    }
}

#[post("/", data = "<login>", format = "json")]
pub(super) async fn login(
    login: Json<LoginData>,
) -> Result<status::Accepted<Json<TokenData>>, status::Custom<&'static str>> {
    let exec = || async {
        let user: Option<Usuario> = database::DATABASE
            .fetch_by_column("username", &login.username)
            .await
            .map_err(|why| Errors::Database(DatabaseError::FindError(why.into())))?;

        let mut user = user.ok_or(Errors::Auth(AuthError::InvalidUserOrPassword))?;

        if user.password != login.password {
            return Err(Errors::Auth(AuthError::InvalidUserOrPassword));
        }

        let token = jwt::create_token(&Tokenzin).map_err(Errors::Auth)?;

        user.jwt = Some(token.clone());

        database::DATABASE
            .update_by_column("username", &mut user)
            .await
            .map_err(|why| Errors::Database(DatabaseError::UpdateErrro(why.into())))?;

        let token = TokenData {
            token,
            username: login.username.clone(),
        };

        Ok(token) as Result<TokenData, Errors>
    };

    let data = exec().await.map_err(|why| match why {
        Errors::Auth(AuthError::InvalidUserOrPassword) => {
            status::Custom(Status::Unauthorized, "Login ou senha incorretos")
        }
        outhers => {
            log::error!("{}", outhers);
            status::Custom(
                Status::InternalServerError,
                "Erro interno, consulte o corno maior",
            )
        }
    })?;

    Ok(status::Accepted(Some(Json(data))))
}

pub(super) struct LoggedUser;

#[async_trait]
impl<'r> FromRequest<'r> for LoggedUser {
    type Error = Errors;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let exec = || async {
            let token = request
                .headers()
                .get("Authorization")
                .next()
                .ok_or(Errors::Auth(AuthError::AuthenticationHeaderNotFound))?;

            let user: Option<Usuario> = database::DATABASE
                .fetch_by_column("jwt", &token)
                .await
                .map_err(|why| Errors::Database(DatabaseError::FindError(why.into())))?;

            if user.is_none() || user.unwrap().jwt == Some("NULL".to_owned()) {
                return Err(Errors::Auth(AuthError::InvalidUserOrPassword));
            }

            Ok(()) as Result<(), Errors>
        };

        match exec().await {
            Ok(_) => Outcome::Success(Self),
            Err(why) => Outcome::Failure((Status::Unauthorized, why)),
        }
    }
}
