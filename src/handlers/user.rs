use std::io::Error;
use std::io::ErrorKind;

use warp::{http::StatusCode, reject, Rejection, Reply};

use crate::config::Config;

use crate::config::DBPool;
use crate::errors::Error::{AuthError, NotCompletedError};
use crate::handlers::auth::validate_credentials;
use crate::models::{
    auth::Credentials,
    user::{NewUser, ValidateEmail},
};

pub async fn me(
    token: String,
    _realm_header: String,
    config: Config,
    db_pool: DBPool,
) -> std::result::Result<impl Reply, Rejection> {
    let user_repo = match config.user_repo(db_pool.clone()).await {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };
    let id = match config.token_service().verify_jwt(token).await {
        Ok(data) => data.claims.sub,
        Err(e) => return Err(e),
    };
    match user_repo.get_user_by_id(id).await? {
        Some(user) => {
            match serde_json::to_string(&user) {
                Ok(user_json) => return Ok(warp::reply::with_status(user_json, StatusCode::OK)),
                Err(_) => return Err(reject::custom(NotCompletedError(ErrorKind::WriteZero))),
            };
        }
        None => {
            return Err(reject::custom(AuthError(Error::from(
                ErrorKind::PermissionDenied,
            ))))
        }
    };
}

pub async fn create_user(
    credentials: Credentials,
    _realm_header: String,
    config: Config,
    db_pool: DBPool,
    body: ValidateEmail,
) -> Result<impl Reply, Rejection> {
    let user_repo = match config.user_repo(db_pool.clone()).await {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };

    // if validate_credentials(&credentials, &user_repo, config.hash_service()).await {
    //     return Err(reject::custom(ExistsError(Error::from(
    //         ErrorKind::AlreadyExists,
    //     ))));
    // };

    match config
        .hash_service()
        .hash_password(credentials.password.clone())
        .await
    {
        Ok(p) => {
            let new_user = NewUser {
                username: credentials.username,
                password_hash: p,
                email: body.email,
            };

            let id = match user_repo.create(new_user).await? {
                Some(id) => id,
                None => return Err(reject::custom(NotCompletedError(ErrorKind::WriteZero))),
            };

            match config.token_service().generate_jwt(id).await {
                Ok(token) => return Ok(warp::reply::with_status(token, StatusCode::OK)),
                Err(e) => return Err(e),
            };
        }
        Err(e) => return Err(e),
    };
}

pub async fn login(
    credentials: Credentials,
    _realm_header: String,
    config: Config,
    db_pool: DBPool,
) -> Result<impl Reply, Rejection> {
    let user_repo = match config.user_repo(db_pool.clone()).await {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };
    let id = match validate_credentials(&credentials, &user_repo, config.hash_service()).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            return Err(reject::custom(AuthError(Error::from(
                ErrorKind::PermissionDenied,
            ))))
        }
        Err(e) => return Err(e),
    };

    match config.token_service().generate_jwt(id).await {
        Ok(token) => return Ok(warp::reply::with_status(token, StatusCode::OK)),
        Err(e) => return Err(e),
    };
}
pub async fn delete_user(
    token: String,
    _realm_header: String,
    config: Config,
    db_pool: DBPool,
) -> Result<impl Reply, Rejection> {
    let user_repo = match config.user_repo(db_pool.clone()).await {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };
    let id = match config.token_service().verify_jwt(token).await {
        Ok(data) => data.claims.sub,
        Err(e) => return Err(e),
    };

    let uuid = match user_repo.validate_id(id).await? {
        Some(id) => id,
        None => {
            return Err(reject::custom(AuthError(Error::from(
                ErrorKind::PermissionDenied,
            ))))
        }
    };

    match user_repo.delete(uuid).await? {
        Some(id) => Ok(warp::reply::with_status(id.to_string(), StatusCode::OK)),
        None => return Err(reject::custom(NotCompletedError(ErrorKind::WriteZero))),
    }
}
