use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres::NoTls, PgConnectionManager};
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;
use warp::{reject, Rejection};

use crate::errors::Error::{DBConnError, DBQueryError};
use crate::models::user::{NewUser, User};

pub struct UserRepository {
    db: Connection<PgConnectionManager<NoTls>>,
}

impl UserRepository {
    pub async fn new(pool: Pool<PgConnectionManager<NoTls>>) -> Result<Self, Rejection> {
        match pool.get().await {
            Ok(db) => return Ok(Self { db }),
            Err(e) => return Err(reject::custom(DBConnError(e))),
        }
    }
    pub async fn create(&self, new_user: NewUser) -> Result<Option<Uuid>, Rejection> {
        let rows = self
            .db
            .query(
                "insert into users (username, email, password_hash) values ($1, $2, $3) returning id",
                &[&new_user.username, &new_user.email, new_user.password_hash.expose_secret()],
            )
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))?;
        if rows.is_empty() {
            ()
        };
        let id: Uuid = rows[0].get(0);
        Ok(Some(id))
    }
    pub async fn delete(&self, id: Uuid) -> Result<Option<Uuid>, Rejection> {
        let rows = self
            .db
            .query("delete from users where id = $1 returning id", &[&id])
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))?;
        if rows.is_empty() {
            ()
        }
        let id: Uuid = rows[0].get(0);
        Ok(Some(id))
    }
    pub async fn get_password_hash(
        &self,
        username: &String,
    ) -> Result<Option<(Secret<String>, Uuid)>, Rejection> {
        match self
            .db
            .query("SELECT * FROM users WHERE username = $1", &[&username])
            .await
        {
            Ok(rows) => {
                if rows.is_empty() || rows.len() == 0 {
                    return Ok(None);
                }
                let pass: String = rows[0].get("password_hash");
                let secret = Secret::new(pass);
                let id: Uuid = rows[0].get("id");
                Ok(Some((secret, id)))
            }
            Err(e) => {
                return Err(reject::custom(DBQueryError(e)));
            }
        }
    }
    pub async fn validate_id(&self, id: Uuid) -> Result<Option<Uuid>, Rejection> {
        match self
            .db
            .query("SELECT id FROM users WHERE id = $1", &[&id])
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))
        {
            Ok(rows) => {
                if rows.is_empty() || rows.len() == 0 {
                    return Ok(None);
                }
                let id: Uuid = rows[0].get("id");
                Ok(Some(id))
            }
            Err(e) => return Err(e),
        }
    }
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, Rejection> {
        match self
            .db
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .await
            .map_err(|e| reject::custom(DBQueryError(e)))
        {
            Ok(rows) => {
                if rows.is_empty() || rows.len() == 0 {
                    return Ok(None);
                }
                let user = User {
                    id: rows[0].get("id"),
                    username: rows[0].get("username"),
                    email: rows[0].get("email"),
                    password_hash: "secret".to_string(),
                    full_name: rows[0].get("full_name"),
                    bio: rows[0].get("bio"),
                    image: rows[0].get("image"),
                    email_verified: rows[0].get("email_verified"),
                    active: rows[0].get("active"),
                };
                Ok(Some(user))
            }
            Err(e) => return Err(e),
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_delete_user() {
    use crate::config::Config;
    use uuid::Uuid;
    let id: Uuid = Uuid::parse_str("a8a36eed-c165-45c8-8969-3f0f7f50fbfb").unwrap();

    let config = Config::from_env().expect("config");
    let db_pool = config.db_pool().expect("db_pool");

    let deleted_id = config
        .user_repo(db_pool)
        .await
        .unwrap()
        .delete(id)
        .await
        .unwrap();

    assert_eq!(id, deleted_id.unwrap());
}
