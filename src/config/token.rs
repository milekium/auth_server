use crate::errors::Error::TokenError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::reject;
use warp::Rejection;

#[derive(Clone)]
pub struct TokenService {
    pub jwt_secret: Arc<String>,
    pub header: Header,
    pub validation: Validation,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    // aud
    // role
    // perms
}

impl TokenService {
    pub async fn generate_jwt(&self, uuid: Uuid) -> Result<String, Rejection> {
        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let now = Utc::now() + Duration::days(1); // Expires in 1 day
        let claims = Claims {
            sub: uuid,
            exp: now.timestamp(),
        };
        match encode(&self.header, &claims, &encoding_key) {
            Ok(token) => return Ok(token),
            Err(e) => return Err(reject::custom(TokenError(e))),
        }
    }
    pub async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>, Rejection> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        match decode::<Claims>(&token, &decoding_key, &self.validation) {
            Ok(c) => return Ok(c),
            Err(e) => return Err(reject::custom(TokenError(e))),
        }
    }
}
