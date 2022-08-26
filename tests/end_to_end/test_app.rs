use actix_web::dev::ServerHandle;
use veil::startup::Application;
use veil::Config;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

const LOCALHOST: &str = "127.0.0.1";

pub struct TestApp {
    config: Config,
    db_url: String,
    port: u16,
    server_handle: ServerHandle,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        let configuration = {
            let mut config = Config::from_env().expect("Failed to read config from env");
            config.db_name = Uuid::new_v4().to_string(); // Use random name for DB
            config.app_name = Uuid::new_v4().to_string();
            config.app_port = 0; // Use random port, assigned by OS
            config.app_host = LOCALHOST.to_string();

            config
        };

        let db_url = postgres_url(&configuration);
        setup_database(&db_url).await.expect("Failed to set up test database");

        let application = Application::build(configuration.clone())
            .await
            .expect("Failed to setup test application");
        let app_port = application.port();
        let server_handle = application.handle();

        let _ = tokio::spawn(application.run());

        TestApp {
            config: configuration,
            db_url,
            port: app_port,
            server_handle,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", LOCALHOST, self.port)
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn stop(&self) {
        self.server_handle.stop(true).await;
        sqlx::Postgres::drop_database(&self.db_url)
            .await
            .expect("Failed to drop test database");
    }
}

async fn setup_database(db_url: &str) -> Result<(), sqlx::Error> {
    sqlx::Postgres::create_database(db_url).await?;
    let db_pool: PgPool = PgPoolOptions::new().connect(db_url).await?;
    sqlx::migrate!().run(&db_pool).await?;
    db_pool.close().await;

    Ok(())
}

fn postgres_url(config: &Config) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    )
}
