// use reqwest::header;
mod common;

// #[tokio::test]
// async fn requests_realm_header_are_rejected() {
//     common::spawn_app().await;

//     let mut headers = header::HeaderMap::new();
//     headers.insert(
//         "WWW-Authenticate",
//         header::HeaderValue::from_static("Basic realm=AuthServer"),
//     );

//     let mut auth_value = header::HeaderValue::from_static("Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
//     auth_value.set_sensitive(true);
//     headers.insert(header::AUTHORIZATION, auth_value);

//     let client = reqwest::Client::builder()
//         .user_agent("vue/v3")
//         .default_headers(headers)
//         .build()
//         .expect("build request should pass");

//     let response = client
//         .get("http://127.0.0.1:3000/signup")
//         .send()
//         .await
//         .expect("Failed to execute request to /signup");
//     assert_eq!(200, response.status().as_u16());

// let text = response.text().await.expect("text extraction fail");

// assert_eq!("200", response);
// }

// #[tokio::test]
// async fn requests_missing_authorization_are_rejected() {
//     common::spawn_app().await;

//     let response = reqwest::Client::new()
//         .get(r#"http://127.0.0.1:3000/me"#)
//         .send()
//         .await
//         .expect(r#"Failed to execute request."#);
//     assert_eq!(401, response.status().as_u16());
// }
