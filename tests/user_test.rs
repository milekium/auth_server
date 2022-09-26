mod common;

#[tokio::test]
async fn test_user() {
    common::spawn_app().await;

    let credentials = common::Credentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };

    //create new user
    let (code, token) = common::singup(credentials.clone()).await;
    assert_eq!(200, code,);
    assert!(token.len() > 0);
    println!("Singup success with token: {}", token);
    //Singup
    let (code, token) = common::login(credentials).await;
    assert_eq!(200, code);

    //Get user info
    // let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjMWRkZDI3ZC03ZDU2LTQ0ZGMtYTY2OS0xOWU4NzAxNjk2ZDgiLCJleHAiOjE2NjQwMTc1MTd9.kbSSDQ9Po-_XVl426SyOMNGCeQx3hl3zLBHlWOFAcik".to_string();
    let (code, user_serialize) = common::me(token.clone()).await;
    assert_eq!(200, code);
    let user: common::User = serde_json::from_str(&user_serialize).unwrap();
    println!("{:?}", &user);
    assert_eq!(user.username.unwrap(), "username".to_string());

    //delete user
    let (code, token) = common::delete(token.clone()).await;
    assert_eq!(200, code);
    println!("Delete User success with token: {}", token);
    assert!(token.len() > 0);
}
