use actix_web::{dev::HttpServiceFactory, get, web, HttpResponse, Responder};

mod authentication;

#[get("./")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lingo API")
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .service(hello)
        .service(authentication::routes())
}
