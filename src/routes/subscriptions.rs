// vim: ts=2 sw=2
//
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

pub async fn subscribe(
  form: web::Form<FormData>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let req_id = Uuid::new_v4();

  let req_span = tracing::info_span!(
    "adding a new subscriber",
    %req_id,
    subscriber_email = %form.email,
    subscriber_name = %form.name
  );

  let _req_span_guard = req_span.enter();

  let query_span =
    tracing::info_span!("saving new subscriber details in the database");

  match sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(pool.get_ref())
  .instrument(query_span)
  .await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => {
      tracing::error!("req {} - failed to execute query: {:?}", req_id, e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
