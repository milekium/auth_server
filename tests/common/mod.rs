use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use validator::Validate;

use authserver::run;
use base64::encode_config;
use reqwest::header;
use tokio::time::sleep;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn spawn_app() {
    let server = run();
    let _ = tokio::task::spawn(server);
    sleep(Duration::from_millis(100)).await;
    println!("100 ms have elapsed");
}
#[allow(dead_code)]
pub async fn singup(credentials: Credentials) -> (u16, String) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "WWW-Authenticate",
        header::HeaderValue::from_static("Basic realm=AuthServer"),
    );
    let credentials_inline = format!("{}:{}", credentials.username, credentials.password);
    let encoded_credentials: &str = &encode_config(credentials_inline, base64::STANDARD);
    let auth_value = format!("Basic {}", encoded_credentials);
    let from_str = header::HeaderValue::from_str(&auth_value).unwrap();
    headers.insert(header::AUTHORIZATION, from_str);

    let mut params = HashMap::new();
    params.insert("email", "milekium@proton.com");

    let client = reqwest::Client::builder()
        .user_agent("vue/v3")
        .default_headers(headers)
        .build()
        .expect("build request should pass");

    let response = client
        .post("http://127.0.0.1:3000/signup")
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request to /signup");

    (
        response.status().as_u16(),
        response.text().await.expect("text extraction fail"),
    )
}
#[allow(dead_code)]
pub async fn login(credentials: Credentials) -> (u16, String) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "WWW-Authenticate",
        header::HeaderValue::from_static("Basic realm=AuthServer"),
    );
    let credentials_inline = format!("{}:{}", credentials.username, credentials.password);
    let encoded_credentials: &str = &encode_config(credentials_inline, base64::STANDARD);
    let auth_value = format!("Basic {}", encoded_credentials);
    let from_str = header::HeaderValue::from_str(&auth_value).unwrap();
    headers.insert(header::AUTHORIZATION, from_str);

    let client = reqwest::Client::builder()
        .user_agent("vue/v3")
        .default_headers(headers)
        .build()
        .expect("build request should pass");

    let response = client
        .post("http://127.0.0.1:3000/login")
        .send()
        .await
        .expect("Failed to execute request to /login");

    (
        response.status().as_u16(),
        response.text().await.expect("text extraction fail"),
    )
}
#[allow(dead_code)]
pub async fn delete(token: String) -> (u16, String) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "WWW-Authenticate",
        header::HeaderValue::from_static("Basic realm=AuthServer"),
    );

    let encoded_credentials: &str = &encode_config(token, base64::STANDARD);
    let auth_value = format!("Basic {}", encoded_credentials);
    let from_str = header::HeaderValue::from_str(&auth_value).unwrap();
    headers.insert(header::AUTHORIZATION, from_str);

    let client = reqwest::Client::builder()
        .user_agent("vue/v3")
        .default_headers(headers)
        .build()
        .expect("build request should pass");

    let response = client
        .delete("http://127.0.0.1:3000/me")
        .send()
        .await
        .expect("Failed to execute request to /delete");

    (
        response.status().as_u16(),
        response.text().await.expect("text extraction fail"),
    )
}

#[allow(dead_code)]
pub async fn me(token: String) -> (u16, String) {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "WWW-Authenticate",
        header::HeaderValue::from_static("Basic realm=AuthServer"),
    );

    let encoded_credentials: &str = &encode_config(token, base64::STANDARD);
    let auth_value = format!("Basic {}", encoded_credentials);
    let from_str = header::HeaderValue::from_str(&auth_value).unwrap();
    headers.insert(header::AUTHORIZATION, from_str);

    let client = reqwest::Client::builder()
        .user_agent("vue/v3")
        .default_headers(headers)
        .build()
        .expect("build request should pass");

    let response = client
        .get("http://127.0.0.1:3000/me")
        .send()
        .await
        .expect("Failed to execute request to /delete");

    (
        response.status().as_u16(),
        response.text().await.expect("text extraction fail"),
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Validate, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
}
