use veil::Config;
use reqwest::Response;
use serde_json::Value;
use url::Url;

pub struct LocalAuth0 {
    config: Config,
    client: reqwest::Client,
}

impl LocalAuth0 {
    pub fn new(config_ref: &Config) -> Self {
        Self {
            config: config_ref.clone(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn new_token_with_permissions(&self, permissions: Vec<String>) -> String {
        self.update_permissions_for_audience(permissions).await;
        self.new_token().await
    }

    // Ask localauth0 server to get a jwt
    pub async fn new_token(&self) -> String {
        let body: String = format!(
            "{{\"client_id\":\"client_id\",\"client_secret\":\"client_secret\",\"audience\":\"{}\",\"grant_type\":\"client_credentials\"}}",
            &self.config.app_name,
        );

        let response: Response = self
            .client
            .post(self.jwt_url())
            .header("Content-type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        let json_value: Value = response.json::<serde_json::Value>().await.unwrap();
        json_value.get("access_token").unwrap().as_str().unwrap().to_string()
    }

    // Ask localauth0 server to add permissions for the config.app_name() audience
    pub async fn update_permissions_for_audience(&self, permissions: Vec<String>) {
        let body: String = format!(
            "{{\"audience\":\"{}\",\"permissions\":[\"{}\"]}}",
            &self.config.app_name,
            permissions.join("\", \"")
        );

        self.client
            .post(self.permissions_url())
            .header("Content-type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
    }

    fn jwt_url(&self) -> Url {
        self.config.authority_url.to_owned().join("/oauth/token").unwrap()
    }

    fn permissions_url(&self) -> Url {
        self.config.authority_url.to_owned().join("/permissions").unwrap()
    }
}
