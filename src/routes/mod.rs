use actix_web::{dev::HttpServiceFactory, get, web, HttpResponse, Responder};

use crate::models::users::AccessToken;

mod authentication;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lingo API")
}

#[get("/authorized")]
async fn authorized(token: AccessToken) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome: {}", token.token))
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .service(hello)
        .service(authorized)
        .service(authentication::routes())
}
