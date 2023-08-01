// a function to generate a jwt token
#[allow(dead_code)]
const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    // role: String,
    exp: usize,
}

pub fn create_jwt(id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id.to_owned(),
        // role: role.to_string(),
        exp: expiration as usize,
    };

    // sign token and return string
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .expect("unable to encode jwt")
}

#[allow(dead_code)]
pub fn validate_jwt(token: &str) -> Result<String, String> {
    let token = token.replace(BEARER, "");

    let validation = Validation {
        algorithms: vec![Algorithm::HS512],
        ..Validation::default()
    };

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    );

    match token_data {
        Ok(data) => Ok(data.claims.sub),
        Err(_) => Err("invalid token".to_string()),
    }
}
