use crate::errors::Error::HashError;

// use rand_core::OsRng;
use secrecy::{ExposeSecret, Secret};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use warp::reject;
use warp::Rejection;

#[derive(Clone)]
pub struct HashService {
    pub salt: SaltString,
}

impl HashService {
    pub async fn hash_password(&self, password: String) -> Result<Secret<String>, Rejection> {
        match Argon2::default().hash_password(password.as_bytes(), &self.salt) {
            Ok(p) => return Ok(Secret::new(p.to_string())),
            Err(e) => return Err(reject::custom(HashError(e))),
        }
    }
    #[allow(dead_code)]
    pub async fn verify_password_hash(
        &self,
        password: String,
        password_hash: Secret<String>,
    ) -> Result<bool, Rejection> {
        let password_hash_phc = PasswordHash::new(password_hash.expose_secret()).unwrap();

        match Argon2::default().verify_password(password.as_bytes(), &password_hash_phc) {
            Ok(_) => return Ok(true),
            Err(e) => return Err(reject::custom(HashError(e))),
        };
    }
}

#[tokio::test]
async fn test_hash_service() {
    use crate::config::Config;

    let config = Config::from_env().expect("Server configuration can be loaded");
    let hashing_service = config.hash_service();
    let hashing_service1 = config.hash_service();

    let password = "uuid".to_string();

    let password_hash = hashing_service
        .hash_password(password.clone())
        .await
        .unwrap();

    let password_hash1 = hashing_service1.hash_password(password).await.unwrap();

    assert_ne!(
        password_hash.expose_secret(),
        password_hash1.expose_secret()
    );
}

#[tokio::test]
async fn test_hash_password() {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    let salt = SaltString::generate(&mut OsRng);

    let hash_service = HashService { salt };
    let password = "uuid".to_string();

    let password_hash = hash_service.hash_password(password).await.unwrap();
    assert_eq!(password_hash.expose_secret().is_empty(), false);
}

#[tokio::test]
async fn test_verify_password_hash() {
    use argon2::password_hash::{rand_core::OsRng, SaltString};

    let salt = SaltString::generate(&mut OsRng);

    let password = String::from("password");

    let hash_service1 = HashService { salt: salt.clone() };
    let password_hash = hash_service1.hash_password(password.clone()).await.unwrap();

    let salt2 = SaltString::generate(&mut OsRng);
    let hash_service2 = HashService { salt: salt2 };
    let password_hash2 = hash_service2.hash_password(password.clone()).await.unwrap();

    let verified = hash_service1
        .verify_password_hash(password.clone(), password_hash.clone())
        .await
        .unwrap();

    let verified2 = hash_service2
        .verify_password_hash(password.clone(), password_hash2.clone())
        .await
        .unwrap();

    assert_eq!(verified, verified2);

    // assert_eq!(
    //     password_hash.expose_secret(),
    //     password_hash2.expose_secret()
    // );
}
