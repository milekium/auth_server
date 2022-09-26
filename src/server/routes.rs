use std::convert::Infallible;

use crate::config::{Config, DBPool};
use crate::errors;
use crate::errors::Error::{AuthError, PathMismatch};
use crate::handlers::auth::{decode_credentials, decode_token};
use crate::handlers::health_handler;
use crate::handlers::user::{create_user, delete_user, login, me};
use crate::models::auth::Credentials;

use std::io::Error;
use std::io::ErrorKind;

use warp::{body, path, reject, Rejection};
use warp::{filters::BoxedFilter, Filter, Reply};

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
fn with_config(config: Config) -> impl Filter<Extract = (Config,), Error = Infallible> + Clone {
    warp::any().map(move || config.clone())
}

fn with_realm_header() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::header::<String>("WWW-Authenticate").and_then(|a: String| async move {
        let realm = a.replace("Basic realm=", "");
        if realm == "AuthServer" {
            Ok(a)
        } else {
            Err(reject::custom(AuthError(Error::from(
                ErrorKind::InvalidData,
            ))))
        }
    })
}
fn with_token_auth_header() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::header::<String>("authorization").and_then(|a: String| async move {
        if let Some(e) = a.strip_prefix("Basic ") {
            match decode_token(String::from(e)).await {
                Ok(a) => return Ok(a),
                Err(_e) => {
                    return Err(reject::custom(AuthError(Error::from(
                        ErrorKind::PermissionDenied,
                    ))))
                }
            };
        } else {
            Err(reject::custom(AuthError(Error::from(
                ErrorKind::PermissionDenied,
            ))))
        }
    })
}
fn with_basic_auth_header() -> impl Filter<Extract = (Credentials,), Error = Rejection> + Copy {
    warp::header::<String>("authorization").and_then(|a: String| async move {
        if let Some(e) = a.strip_prefix("Basic ") {
            match decode_credentials(String::from(e)).await {
                Ok(a) => return Ok(a),
                Err(e) => return Err(e),
            };
        } else {
            Err(reject::custom(AuthError(Error::from(
                ErrorKind::InvalidInput,
            ))))
        }
    })
}

pub fn make_routes(config: Config, db_pool: DBPool) -> BoxedFilter<(impl Reply,)> {
    let health = warp::path("health")
        .and(with_db(db_pool.clone()))
        .and_then(health_handler);

    let user_agent = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    let signup = warp::post().and(
        path!("signup")
            .and(with_basic_auth_header())
            .and(with_realm_header())
            .and(with_config(config.clone()))
            .and(with_db(db_pool.clone()))
            .and(body::form())
            .and_then(create_user),
    );
    let delete = warp::delete().and(
        path!("me")
            .and(with_token_auth_header())
            .and(with_realm_header())
            .and(with_config(config.clone()))
            .and(with_db(db_pool.clone()))
            .and_then(delete_user),
    );
    let me = warp::get().and(
        path!("me")
            .or_else(|_| async { Err(reject::custom(PathMismatch)) })
            .and(with_token_auth_header())
            .and(with_realm_header())
            .and(with_config(config.clone()))
            .and(with_db(db_pool.clone()))
            .and_then(me),
    );
    let login = warp::post().and(
        path!("login")
            .or_else(|_| async { Err(reject::custom(PathMismatch)) })
            .and(with_basic_auth_header())
            .and(with_realm_header())
            .and(with_config(config.clone()))
            .and(with_db(db_pool.clone()))
            .and_then(login),
    );

    health
        .or(signup)
        .or(login)
        .or(delete)
        .or(user_agent)
        .or(me)
        .recover(errors::handle_rejection)
        .boxed()
}
