use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jwks_client_rs::JwksClientError;
use serde::{Deserialize, Serialize};

use super::app_data::AppData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    permissions: Vec<String>,
}

impl Claims {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|x| x == permission)
    }
}

pub async fn validate_token(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let app_data = req
        .app_data::<Data<AppData>>()
        .cloned()
        .expect("AppData is missing");

    let decoded: Result<Claims, JwksClientError> = app_data
        .jwks_client()
        .decode(credentials.token(), &[app_data.config().app_name.clone()])
        .await;

    match decoded {
        Ok(claims) => {
            tracing::debug!("Claims recovered from token: {:?}", claims);
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(error) => {
            tracing::info!("Token decode error: {:?}", error);
            Err(ErrorUnauthorized("Token is not valid"))
        }
    }
}
