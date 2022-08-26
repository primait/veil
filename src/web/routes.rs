use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::{
    authentication::Claims,
    graphql::{error::ErrorCode, GreetingSchema},
};

pub async fn graphql(
    schema: web::Data<GreetingSchema>,
    http_request: HttpRequest,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    let claims_opt = { http_request.extensions().get::<Claims>().cloned() };
    if let Some(claims) = claims_opt {
        schema
            .execute(graphql_request.into_inner().data(claims))
            .await
            .into()
    } else {
        GraphQLResponse::from(
            async_graphql::Response::from_errors(vec![async_graphql::ServerError::new(
                "User is unauthorized",
                None,
            )])
            .extension(
                "code",
                async_graphql::Value::String(ErrorCode::Unauthorized.as_ref().to_string()),
            ),
        )
    }
}

pub async fn check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": "ok"}"#)
}
