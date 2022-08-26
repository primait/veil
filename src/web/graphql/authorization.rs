use async_graphql::{async_trait, Context, Error, ErrorExtensions, Guard};
use strum_macros::{AsRefStr, EnumIter};

use crate::web::authentication::Claims;

use super::error::ErrorCode;

#[derive(AsRefStr, EnumIter)]
pub enum Permission {
    #[strum(serialize = "greeting:read")]
    ReadGreeting,
}

pub struct Authorization {
    permission: Permission,
}

impl Authorization {
    pub fn with_permission(permission: Permission) -> Self {
        Self { permission }
    }
}

#[async_trait::async_trait]
impl Guard for Authorization {
    async fn check(&self, context: &Context<'_>) -> async_graphql::Result<()> {
        match context.data_opt::<Claims>() {
            Some(claims) if claims.has_permission(self.permission.as_ref()) => Ok(()),
            _ => Err(Error::new("User is unauthorized")
                .extend_with(|_, e| e.set("code", ErrorCode::Unauthorized.as_ref()))),
        }
    }
}
