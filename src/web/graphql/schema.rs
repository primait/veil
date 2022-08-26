use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::{types::time::OffsetDateTime, PgPool};

use super::authorization::{Authorization, Permission};

pub type GreetingSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[tracing::instrument(skip_all)]
    #[graphql(guard = "Authorization::with_permission(Permission::ReadGreeting)")]
    async fn greeting<'ctx>(&self, ctx: &Context<'ctx>) -> async_graphql::Result<String> {
        let pool = ctx.data_unchecked::<PgPool>();

        // Just an example of querying the DB
        // In real life you probably want to extract this somewhere else
        let (now,): (OffsetDateTime,) = sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;

        tracing::info!("Greeting someone");
        prima_datadog::incr!("veil.greetings");

        Ok(format!("Hello, world! Today is {}", now.date()))
    }
}
