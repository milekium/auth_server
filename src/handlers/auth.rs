use crate::config::hash::HashService;
use crate::db::user::UserRepository;
use crate::{
    errors::Error::{AuthError, NotCompletedError, NotFoundError},
    models::auth::Credentials,
};
use base64::decode_config;
use std::io::Error;
use std::io::ErrorKind;
use uuid::Uuid;
use warp::reject;
use warp::Rejection;

pub async fn decode_token(base64encoded_segment: String) -> Result<String, Rejection> {
    match decode_config(base64encoded_segment, base64::STANDARD) {
        Ok(token_bits) => match String::from_utf8(token_bits) {
            Ok(token) => return Ok(token),
            Err(_e) => return Err(reject::custom(NotCompletedError(ErrorKind::InvalidInput))),
        },

        Err(_) => Err(reject::custom(AuthError(Error::from(
            ErrorKind::PermissionDenied,
        )))),
    }
}
pub async fn decode_credentials(base64encoded_segment: String) -> Result<Credentials, Rejection> {
    match decode_config(base64encoded_segment, base64::STANDARD) {
        Ok(decoded_credentials) => {
            let credentials_bytes = String::from_utf8(decoded_credentials).unwrap();
            let credentials: Vec<&str> = credentials_bytes.splitn(2, ":").collect();
            let username = String::from(credentials[0]);
            let password = String::from(credentials[1]);

            Ok(Credentials { username, password })
        }
        Err(_e) => Err(reject::custom(AuthError(Error::from(
            ErrorKind::InvalidInput,
        )))),
    }
}

pub async fn validate_credentials(
    credentials: &Credentials,
    user_repo: &UserRepository,
    hash_service: HashService,
) -> Result<Option<Uuid>, Rejection> {
    println!("validate_credentials 1");

    let (password_hash, id) = match user_repo.get_password_hash(&credentials.username).await {
        Ok(Some((pass, id))) => (pass, id),
        Ok(None) => return Err(reject::custom(NotFoundError(ErrorKind::NotFound))),
        Err(e) => return Err(e),
    };
    println!("validate_credentials 2");

    match hash_service
        .verify_password_hash(credentials.password.clone(), password_hash)
        .await
    {
        Ok(valid) => {
            if valid {
                println!("validate_credentials 3");
                return Ok(Some(id));
            };
            return Ok(None);
        }
        Err(e) => return Err(e),
    }
}

#[tokio::test]
async fn test_decode_credentials() {
    use base64::encode_config;
    let encoded_credentials: &str = &encode_config(b"username:password", base64::STANDARD);
    let auth_value = format!("{}", encoded_credentials);
    assert_eq!(auth_value, "dXNlcm5hbWU6cGFzc3dvcmQ=");

    let decoded = decode_credentials(auth_value).await.unwrap();
    assert_eq!(decoded.username, "username");
    assert_eq!(decoded.password, "password");
}
