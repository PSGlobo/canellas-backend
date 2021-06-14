use fake::{
    faker::internet::en::{Password, SafeEmail},
    Fake,
};
use reqwest::Method;
use serde_json::json;

use crate::helpers;

#[derive(Debug, serde::Deserialize)]
struct RespBody {
    id: i32,
    email: String,
}

#[actix_rt::test]
async fn create_user_works() {
    let email: String = SafeEmail().fake::<String>();
    let password: String = Password(8..30).fake::<String>();
    let payload = json!({
        "email": email,
        "password": password,
    });

    let client = helpers::test_client().await;

    let response = client
        .request(Method::POST, "users", &payload)
        .send()
        .await
        .unwrap();

    dbg!(response.status());
    assert!(response.status().is_success());

    let body: RespBody = response.json().await.unwrap();

    assert_eq!(email, body.email);

    let response = client
        .request(
            Method::GET,
            format!("users/{}", body.id).as_str(),
            &serde_json::Value::Null,
        )
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let json: serde_json::Value = response.json().await.unwrap();
    let received = json["email"].as_str().unwrap();
    assert_eq!(received, email);
}
