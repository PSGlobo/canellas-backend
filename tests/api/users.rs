use fake::{
    faker::internet::en::{Password, SafeEmail},
    Fake,
};
use reqwest::Method;
use serde_json::json;

use crate::helpers;

#[derive(Debug, serde::Deserialize)]
struct RespBody {
    id: String,
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
    assert!(!body.id.is_empty());
}
