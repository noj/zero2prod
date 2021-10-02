// vim: ts=2 sw=2
//
use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}
