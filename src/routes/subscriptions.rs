use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FromData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
