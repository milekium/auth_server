mod common;

#[tokio::test]
#[ignore]
async fn test_signup_new_user() {
    common::spawn_app().await;
    let credentials = common::Credentials {
        username: "test".to_string(),
        password: "password".to_string(),
    };
    let (code, token) = common::singup(credentials).await;
    assert_eq!(200, code);
    println!("singup success with token: {}", token);
    assert!(token.len() > 0);
}

#[tokio::test]
#[ignore]
async fn test_delete_user() {
    common::spawn_app().await;
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhOGEzNmVlZC1jMTY1LTQ1YzgtODk2OS0zZjBmN2Y1MGZiZmIiLCJleHAiOjE2NjM5Mzg3MzJ9.t2grAjyDW6_3ftzT0daWh8rhfuKDyVmPmO1lFqCvLeM".to_string();
    let (code, token) = common::delete(token).await;
    assert_eq!(200, code);
    println!("singup success with token: {}", token);
    assert!(token.len() > 0);
}

#[tokio::test]
#[ignore]
async fn test_login() {
    common::spawn_app().await;
    let credentials = common::Credentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };
    let (code, token) = common::login(credentials).await;

    assert_eq!(404, code);
    println!("login success with token: {}", token);
    assert!(token.len() > 0);
}

#[tokio::test]
// #[ignore]
async fn test_me_with_expired_token() {
    common::spawn_app().await;
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhOGEzNmVlZC1jMTY1LTQ1YzgtODk2OS0zZjBmN2Y1MGZiZmIiLCJleHAiOjE2NjM5Mzg3MzJ9.t2grAjyDW6_3ftzT0daWh8rhfuKDyVmPmO1lFqCvLeM".to_string();
    let (code, token) = common::me(token).await;

    assert_eq!(400, code);
    println!("Error message: {}", token);
    assert!(token.len() > 0);
}
