use std::collections::BTreeMap;

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;

const SECRET: &str = "123";

use crate::errors::AuthError;

pub trait JwtConvetClaim {
    fn set_clains(&self) -> BTreeMap<&'static str, String>;
    fn get_values_from_clain(data: &BTreeMap<String, String>) -> Result<Box<Self>, AuthError>;
}

fn get_key() -> Result<Hmac<Sha384>, AuthError> {
    Hmac::new_from_slice(SECRET.as_bytes()).map_err(|_| AuthError::InvalidKeyLenght)
}

pub fn create_token(data: &impl JwtConvetClaim) -> Result<String, AuthError> {
    let header = Header {
        algorithm: jwt::AlgorithmType::Hs384,
        ..Default::default()
    };

    let clain = data.set_clains();
    let token = Token::new(header, clain)
        .sign_with_key(&get_key()?)
        .map_err(|why| AuthError::JwtSerializationError(why.into()))?;
    Ok(token.as_str().into())
}

pub fn get_data<T: JwtConvetClaim>(token: &str) -> Result<Box<T>, AuthError> {
    let token: Token<Header, BTreeMap<String, String>, _> =
        VerifyWithKey::verify_with_key(token, &get_key()?)
            .map_err(|why| AuthError::JwtDeserializationError(why.into()))?;
    let header = token.header();
    let clain = token.claims();

    if header.algorithm.ne(&AlgorithmType::Hs384) {
        return Err(AuthError::InvalidAugorithm);
    }

    let data = T::get_values_from_clain(clain)?;
    Ok(data)
}
