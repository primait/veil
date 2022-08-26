use crate::end_to_end::test_app::TestApp;
use reqwest::Response;

#[actix_web::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;

    let client = reqwest::Client::new();
    let response: Response = client
        .get(&format!("http://{}/check", app.address()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), r#"{"status": "ok"}"#);

    app.stop().await;
}
