use jwks_client_rs::{source::WebSource, JwksClient};

use crate::Config;

pub struct AppData {
    config: Config,
    jwks_client: JwksClient<WebSource>,
}

impl AppData {
    pub fn new(config: Config, jwks_client: JwksClient<WebSource>) -> AppData {
        AppData {
            config,
            jwks_client,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn jwks_client(&self) -> &JwksClient<WebSource> {
        &self.jwks_client
    }
}
