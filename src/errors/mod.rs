use std::convert::Infallible;

use jsonwebtoken::errors::ErrorKind;
use mobc_postgres::tokio_postgres;
use serde::Serialize;
use thiserror::Error;
use warp::{hyper::StatusCode, Rejection, Reply};

// #[derive(Debug)]
// pub struct PathMismatch;

// impl warp::reject::Reject for PathMismatch {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error connecting DB: {0}")]
    DBConnError(mobc::Error<tokio_postgres::Error>),
    // #[error("error reading file: {0}")]
    // ReadFileError(#[from] std::io::Error),
    #[error("error reading authorization header")]
    AuthError(std::io::Error),
    #[error("error Resource Already Exists")]
    ExistsError(std::io::Error),
    #[error("error Token Generation")]
    TokenError(jsonwebtoken::errors::Error),
    #[error("error Hash Generation")]
    HashError(argon2::password_hash::Error),
    #[error("Path not found")]
    PathMismatch,
    #[error("Operation Could Not Be Completed")]
    NotCompletedError(std::io::ErrorKind),
    #[error("Invalid Input Request")]
    InputError(std::io::ErrorKind),
    #[error("Entity Not found")]
    NotFoundError(std::io::ErrorKind),
}

impl warp::reject::Reject for Error {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;
    eprintln!("unhandled error: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::PathMismatch => {
                code = StatusCode::NOT_FOUND;
                message = "Not Found";
            }
            Error::DBQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request";
            }
            Error::NotFoundError(_) => {
                code = StatusCode::NOT_FOUND;
                message = "Entity Not Found";
            }
            Error::InputError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Invalid Input";
            }
            Error::AuthError(_) => {
                code = StatusCode::UNAUTHORIZED;
                message = "Not authorized";
            }
            Error::NotCompletedError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Operation Could Not Be Completed";
            }
            Error::ExistsError(_) => {
                code = StatusCode::CONFLICT;
                message = "Resource Already Exists";
            }
            Error::TokenError(te) => match te.kind() {
                ErrorKind::ExpiredSignature => {
                    code = StatusCode::BAD_REQUEST;
                    message = "Bad Requestdddd";
                }
                _ => {
                    code = StatusCode::BAD_REQUEST;
                    message = "Generation Token Error";
                }
            },
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}
