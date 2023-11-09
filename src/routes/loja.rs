use actix_web::{get, HttpResponse, Responder};


#[get("/")]
pub async fn get_loja() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}