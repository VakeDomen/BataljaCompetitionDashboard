use std::env;

use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey, errors::Error};

use crate::{models::user::User, db::operations_users::get_user_by_username};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub sub: String,
   pub exp: usize,
}

pub fn encode_jwt(user_id: String) -> Result<String, Error> {
    let claims = Claims{ sub: user_id, exp: (60 * 60 * 10000000) };
    let secret = env::var("JWT_SECRET").expect("Missing the JWT_SECRET environment variable.");
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    )
}

pub fn decode_jwt(token: String) -> Option<String> {
    let secret = env::var("JWT_SECRET").expect("Missing the JWT_SECRET environment variable.");
    match decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(Algorithm::HS256)
    ) {
        Ok(data) => Some(data.claims.sub),
        Err(e) =>  {
            println!("Error decoding JTW token: {:#?}", e.to_string());
            None
        }
    }
}

pub fn exchange_token_for_user(token: BearerAuth) -> Option<User> {
    let email = match decode_jwt(token.token().to_string()) {
        Some(uid) => uid,
        None => return None,
    };

    match get_user_by_username(email) {
        Ok(user) => Some(user),
        Err(e) => {
            eprintln!("[JWT exchange_token_for_user] Error finding user: {:#?}", e);
            None
        }
    }
}