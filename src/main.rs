// vim: ts=2 sw=2
//
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let conf = get_configuration().expect("failed to read config");

  println!("{}", conf.database.connection_string());

  let connection_pool = PgPool::connect(&conf.database.connection_string())
    .await
    .expect("Failed to connect to Postgres");

  let address = format!("127.0.0.1:{}", conf.application_port);
  let listener = TcpListener::bind(address)?;
  run(listener, connection_pool)?.await
}
