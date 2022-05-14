// vim: ts=2 sw=2
//
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let subscriber =
    get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
  init_subscriber(subscriber);

  let conf = get_configuration().expect("failed to read config");

  let connection_pool =
    PgPool::connect(&conf.database.connection_string().expose_secret())
      .await
      .expect("Failed to connect to Postgres");

  let address = format!("127.0.0.1:{}", conf.application_port);
  tracing::info!("start listening on {}", address);
  let listener = TcpListener::bind(address)?;
  run(listener, connection_pool)?.await
}
