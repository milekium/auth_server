extern crate authserver;

use authserver::errors::Error;
use authserver::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let config = Config::from_env().expect("Server configuration can be loaded");

    // let db_pool = config.db_pool().expect("Database Pool can be created");

    // run(config, db_pool).await;
    run().await
}
