pub(crate) mod auth;
pub(crate) mod user;

use crate::config::{Config, DBPool};
use crate::errors::Error::{DBConnError, DBQueryError};

use warp::{http::StatusCode, reject, Rejection, Reply};

pub async fn health_handler(db_pool: DBPool) -> std::result::Result<impl Reply, Rejection> {
    let db = Config::db_conn(&db_pool)
        .await
        .map_err(|e| reject::custom(DBConnError(e)))?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}
