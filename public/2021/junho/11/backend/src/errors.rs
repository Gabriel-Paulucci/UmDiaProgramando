use std::fmt::Display;

#[derive(Debug)]
pub enum Errors {
    Database(DatabaseError),
    Auth(AuthError),
}

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(GenericError),
    CreateDatabaseFailed(GenericError),
    FindError(GenericError),
    UpdateErrro(GenericError),
}

#[derive(Debug)]
pub enum AuthError {
    JwtSerializationError(GenericError),
    JwtDeserializationError(GenericError),
    JwtClaimError(GenericError),
    InvalidKeyLenght,
    InvalidAugorithm,
    JwtParseErro(GenericError),
    InvalidUserOrPassword,
    AuthenticationHeaderNotFound,
}
