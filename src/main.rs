// vim: ts=2 sw=2
//
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("zero2prod".into(), "info".into());
  init_subscriber(subscriber);

  let conf = get_configuration().expect("failed to read config");

  tracing::info!("{}", conf.database.connection_string());

  let connection_pool = PgPool::connect(&conf.database.connection_string())
    .await
    .expect("Failed to connect to Postgres");

  let address = format!("127.0.0.1:{}", conf.application_port);
  let listener = TcpListener::bind(address)?;
  run(listener, connection_pool)?.await
}
