use std::sync::Arc;

use crate::end_to_end::{localauth0::LocalAuth0, test_app::TestApp};
use actix_web::http;
use reqwest::Response;
use time::macros::format_description;
use time::OffsetDateTime;

#[actix_web::test]
async fn unauthorized_on_call_with_no_token() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();
    let response: Response = client
        .post(&format!("http://{}/graphql", app.address()))
        .body(r#"{"query":"query {\n\tgreeting\n}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);

    app.stop().await;
}

#[actix_web::test]
async fn unauthorized_on_call_with_missing_permissions() {
    let app = TestApp::spawn().await;

    let localauth0 = LocalAuth0::new(&app.config());
    let jwt = localauth0.new_token().await;

    let client = reqwest::Client::new();
    let response: Response = client
        .post(&format!("http://{}/graphql", app.address()))
        .header("Authorization", format!("Bearer {}", jwt))
        .body(r#"{"query":"{ greeting }"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response_text = response.text().await.expect("Failed to get text from response");

    let graphql_response: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to deserialize response");

    assert_eq!(graphql_response["data"], serde_json::Value::Null);
    assert_eq!(graphql_response["errors"][0]["extensions"]["code"], "UNAUTHORIZED");

    app.stop().await;
}

#[actix_web::test]
async fn unauthorized_on_call_with_wrong_permissions() {
    let app = TestApp::spawn().await;

    let localauth0 = LocalAuth0::new(&app.config());
    let jwt = localauth0
        .new_token_with_permissions(vec!["wrong:permission".to_string()])
        .await;

    let client = reqwest::Client::new();
    let response: Response = client
        .post(&format!("http://{}/graphql", app.address()))
        .header("Authorization", format!("Bearer {}", jwt))
        .body(r#"{"query":"{ greeting }"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response_text = response.text().await.expect("Failed to get text from response");

    let graphql_response: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to deserialize response");

    assert_eq!(graphql_response["data"], serde_json::Value::Null);
    assert_eq!(graphql_response["errors"][0]["extensions"]["code"], "UNAUTHORIZED");

    app.stop().await;
}

#[actix_web::test]
async fn authorized_on_call_with_correct_permissions() {
    let app = TestApp::spawn().await;

    let localauth0 = LocalAuth0::new(&Arc::new(app.config().clone()));
    let jwt = localauth0
        .new_token_with_permissions(vec!["greeting:read".to_string()])
        .await;

    let client = reqwest::Client::new();
    let response: Response = client
        .post(&format!("http://{}/graphql", app.address()))
        .header("Authorization", format!("Bearer {}", jwt))
        .body(r#"{"query":"{ greeting }"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let response_text = response.text().await.expect("Failed to get text from response");

    let graphql_response: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to deserialize response");

    let now = OffsetDateTime::now_utc()
        .format(format_description!("[year]-[month]-[day]"))
        .unwrap();

    assert_eq!(
        graphql_response["data"]["greeting"],
        format!("Hello, world! Today is {}", now)
    );

    app.stop().await;
}
