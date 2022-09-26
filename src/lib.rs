mod config;
mod db;
pub mod errors;
mod handlers;
mod models;
mod server;

use crate::config::Config;
use crate::errors::Error;
use crate::server::routes::make_routes;

pub async fn run() -> Result<(), Error> {
    let config = Config::from_env().expect("Server configuration can be loaded");

    let db_pool = config.db_pool().expect("Database Pool can be created");

    let server = warp::serve(make_routes(config.clone(), db_pool)).run((config.host, config.port));

    Ok(server.await)
}
