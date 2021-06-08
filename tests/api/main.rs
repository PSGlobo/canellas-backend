use reqwest::Method;
use serde_json::Value;

pub mod helpers;

#[actix_rt::test]
async fn health_check_works() {
    let response = helpers::test_client()
        .await
        .request(Method::GET, "health_check", &Value::Null)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
