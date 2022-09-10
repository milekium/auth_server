use std::time::Duration;

use authserver::run;
use tokio::time::sleep;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

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
        .expect("df")
        .text()
        .await
        .unwrap();

    assert_eq!("Hello sean, whose agent is reqwest/v0.8.6", json);
}

// Launch our application in the background ~somehow~
async fn spawn_app() {
    let server = run();
    let _ = tokio::task::spawn(server);
    sleep(Duration::from_millis(100)).await;
    println!("100 ms have elapsed");
}
