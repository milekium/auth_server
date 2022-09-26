mod common;

#[tokio::test]
async fn health_check_works() {
    common::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/health")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn headers_check_works() {
    let client = reqwest::Client::builder()
        .user_agent("reqwest/v0.8.6")
        .build()
        .expect("build ");

    let json = client
        .get("http://127.0.0.1:3000/hello/sean")
        .send()
        .await
        .expect("request not pass")
        .text()
        .await
        .unwrap();

    assert_eq!("Hello sean, whose agent is reqwest/v0.8.6", json);
}
