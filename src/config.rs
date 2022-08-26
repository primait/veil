use anyhow::Context;
use serde::Deserialize;
use serde_with::serde_as;
use std::time::Duration;
use url::Url;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub datadog_from_address: String,
    pub datadog_to_address: String,
    pub app_env: String,
    pub authority_url: Url,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    pub jwks_client_timeout: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    pub jwks_client_connect_timeout: Duration,
    pub opentelemetry_url: Url,
    #[serde(alias = "cargo_pkg_version")]
    pub version: String,
    #[serde(alias = "cargo_pkg_name")]
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub db_user: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_password: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        envy::from_env::<Config>().context("Unable to load configuration from env")
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.db_user, &self.db_password, &self.db_host, &self.db_port, &self.db_name
        )
    }

    pub fn jwks_well_known_url(&self) -> Url {
        self.authority_url
            .clone()
            .join(".well-known/jwks.json")
            // This should never happen since authority_url is checked from Config
            .expect("Not a valid path for jwks.json")
    }

    pub fn jwks_client_timeout(&self) -> Duration {
        self.jwks_client_timeout
    }

    pub fn jwks_client_connect_timeout(&self) -> Duration {
        self.jwks_client_connect_timeout
    }
}
