use crate::web::app_data::AppData;
use crate::web::authentication::validate_token;
use crate::web::graphql::QueryRoot;
use crate::web::routes::{check, graphql};
use crate::Config;
use actix_web::dev::{Server, ServerHandle};
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use jwks_client_rs::source::WebSource;
use jwks_client_rs::JwksClient;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
    db_pool: PgPool,
}

impl Application {
    pub async fn build(config: Config) -> anyhow::Result<Self> {
        let db_pool: PgPool = PgPoolOptions::new().connect(&config.database_url()).await?;

        let address = format!("{}:{}", config.app_host, config.app_port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();

        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(db_pool.clone())
            .finish();

        let server = HttpServer::new(move || {
            let auth = HttpAuthentication::bearer(validate_token);

            let web_source: WebSource = WebSource::builder()
                .with_timeout(config.jwks_client_timeout())
                .with_connect_timeout(config.jwks_client_connect_timeout())
                .build(config.jwks_well_known_url())
                .expect("Failed to configure web source");

            App::new()
                .wrap(TracingLogger::default())
                .route("/check", web::get().to(check))
                .service(
                    web::resource("/graphql")
                        .guard(guard::Post())
                        .to(graphql)
                        .wrap(auth),
                )
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(AppData::new(
                    config.clone(),
                    JwksClient::builder().build(web_source),
                )))
        })
        .listen(listener)?
        .run();

        Ok(Self {
            port,
            server,
            db_pool,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await?;
        self.db_pool.close().await;
        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn handle(&self) -> ServerHandle {
        self.server.handle()
    }
}
