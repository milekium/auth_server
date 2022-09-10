use std::net::Ipv4Addr;
use std::str::FromStr;

use config::ConfigError;
use dotenv::dotenv;
use serde::Deserialize;

use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::Config as Conn_config;
use tokio_postgres::{Error, NoTls};

use std::time::Duration;

pub(crate) type DBPool = Pool<PgConnectionManager<NoTls>>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
    pub db_pool_max_open: u64,
    pub db_pool_max_idle: u64,
    pub db_pool_timeout_seconds: u64,
}

impl Config {
    // #[instrument]
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        // let mailcoach_api_token = std::env::var("MAILCOACH_API_TOKEN").expect("MAILCOACH_API_TOKEN must be set.");

        // tracing_subscriber::fmt()
        //     .with_env_filter(EnvFilter::from_default_env())
        //     .init();

        // info!("Loading configuration");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
    }

    pub fn db_pool(&self) -> Result<Pool<PgConnectionManager<NoTls>>, mobc::Error<Error>> {
        let config = Conn_config::from_str(&self.database_url).unwrap();

        let manager = PgConnectionManager::new(config, NoTls);
        Ok(Pool::builder()
            .max_open(self.db_pool_max_open)
            .max_idle(self.db_pool_max_idle)
            .get_timeout(Some(Duration::from_secs(self.db_pool_timeout_seconds)))
            .build(manager))
    }
    pub async fn db_conn(
        db_pool: &Pool<PgConnectionManager<NoTls>>,
    ) -> Result<Connection<PgConnectionManager<NoTls>>, mobc::Error<tokio_postgres::Error>> {
        db_pool.get().await
    }
}
