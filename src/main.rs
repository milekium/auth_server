mod config;
mod errors;
mod handlers;
mod server;

use crate::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("Server configuration can be loaded");

    let db_pool = config.db_pool().expect("Database Pool can be created");

    server::start((config.host, config.port), db_pool).await;
}
