pub mod hash;
pub mod token;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;

use argon2::password_hash::{rand_core::OsRng, SaltString};
use config::ConfigError;
use dotenv::dotenv;
use jsonwebtoken::{Algorithm, Header, Validation};
use serde::Deserialize;

use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::Config as Conn_config;
use tokio_postgres::{Error, NoTls};
use warp::Rejection;

use std::time::Duration;

use hash::HashService;
use token::TokenService;

use crate::db::user::UserRepository;

pub(crate) type DBPool = Pool<PgConnectionManager<NoTls>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
    pub jwt_signing_key: String,
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

    pub fn hash_service(&self) -> HashService {
        let salt = SaltString::generate(&mut OsRng);
        HashService { salt }
    }
    pub fn token_service(&self) -> TokenService {
        let header = Header {
            kid: Some(self.jwt_signing_key.to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };
        let validation = Validation::new(Algorithm::HS512);
        TokenService {
            jwt_secret: Arc::new(self.jwt_secret.clone()),
            header,
            validation,
        }
    }
    pub async fn user_repo(
        &self,
        db_pool: Pool<PgConnectionManager<NoTls>>,
    ) -> Result<UserRepository, Rejection> {
        match UserRepository::new(db_pool).await {
            Ok(repo) => return Ok(repo),
            Err(e) => return Err(e),
        }
    }
}

#[tokio::test]
async fn test_create_token() {
    use uuid::Uuid;
    let config = Config::from_env().unwrap();
    let tokeniser = config.token_service();
    let uuid = Uuid::new_v4();

    let token = tokeniser.generate_jwt(uuid).await.unwrap();
    println!("new token: {:?}", token);
    assert_eq!(token.is_empty(), false);
}

#[tokio::test]
async fn test_verify_token() {
    use crate::config::token::Claims;
    use jsonwebtoken::{Header, TokenData};
    use uuid::Uuid;

    let config = Config::from_env().unwrap();
    let tokeniser = config.token_service();
    let uuid = Uuid::new_v4();
    let token = tokeniser.generate_jwt(uuid).await.unwrap();

    let verified_token = tokeniser.verify_jwt(token).await.unwrap();

    let claims_data = Claims { sub: uuid, exp: 30 };

    let token_data = TokenData {
        claims: claims_data,
        header: Header::default(),
    };

    assert_eq!(verified_token.claims.sub, token_data.claims.sub);
}

#[tokio::test]
async fn get_token_info() {
    use crate::config::Config;
    use uuid::Uuid;

    use std::str::FromStr;
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6Im15LWhlYWRlci1zaWduaW5nLWtleS10by1jaGFuZ2UtaW4tcHJvZCJ9.eyJzdWIiOiI0MzdhNGFkZC1jMGQyLTRjZmItYjA0Yy0wN2U3YjM2Y2VhYmIiLCJleHAiOjE2NjQyNzE0OTB9.RRgxwaE-jmVMp6SOsF-qWy5KDxqj57uuFrwGxhlrBWQH3l-Lu1cGCV1LwZXUXl-9A9XF7GFASmDQBPorP9TaBQ".to_string();
    let config = Config::from_env().expect("config");

    let service = config.token_service();
    let token_data = match service.verify_jwt(token).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {:?}", e);
            panic!("eroor")
        }
    };
    assert_eq!(
        token_data.claims.sub,
        Uuid::from_str("a8a36eed-c165-45c8-8969-3f0f7f50fbfb").unwrap()
    );
}
