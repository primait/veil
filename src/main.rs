use dotenv::dotenv;
use prima_datadog::error::Error;
use prima_datadog::{
    configuration::{Environment, PrimaConfiguration},
    Datadog,
};
use prima_tracing::Uninstall;
use veil::startup::Application;
use veil::Config;

use std::str::FromStr;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let configuration = Config::from_env()?;

    init_datadog(&configuration).expect("Failed to initialize Datadog client");
    let _guard = init_tracing(&configuration);

    tracing::info!(
        "Starting {} in {} at {}:{}",
        &configuration.app_name,
        &configuration.app_env,
        &configuration.app_host,
        &configuration.app_port
    );

    let application = Application::build(configuration)
        .await
        .expect("Failed to setup application");

    application.run().await.expect("Failed to start server");

    Ok(())
}

fn init_datadog(configuration: &Config) -> Result<(), Error> {
    let datadog_configuration = PrimaConfiguration::new(
        &configuration.datadog_to_address,
        &configuration.datadog_from_address,
        &configuration.app_name,
        Environment::from_str(&configuration.app_env)?,
    );
    Datadog::init(datadog_configuration)
}

fn init_tracing(configuration: &Config) -> Uninstall {
    let environment = prima_tracing::Environment::from_str(&configuration.app_env)
        .expect("Failed to parse tracing environment");
    let subscriber = prima_tracing::configure_subscriber(
        prima_tracing::builder(&configuration.app_name)
            .with_env(environment)
            .with_version(configuration.version.clone())
            .with_telemetry(
                configuration.opentelemetry_url.to_string(),
                configuration.app_name.clone(),
            )
            .build(),
    );
    prima_tracing::init_subscriber(subscriber)
}