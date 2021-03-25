use actix_web::{dev::HttpServiceFactory, get, web, HttpResponse, Responder};

use crate::models::users::AccessToken;

mod authentication;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lingo API")
}

#[get("/authorized")]
async fn authorized(
    data: web::Data<crate::WebState>,
    token: AccessToken,
) -> Result<HttpResponse, actix_web::Error> {
    let user = token.get_user(&data.db).await?;

    Ok(HttpResponse::Ok().body(format!("Welcome: {}", &user.username)))
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .service(hello)
        .service(authorized)
        .service(authentication::routes())
}
