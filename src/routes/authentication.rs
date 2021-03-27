use crate::models::{self, users::AccessToken, Model, ResponseError};
use actix_web::{
    dev::HttpServiceFactory, get, http::StatusCode, post, web, HttpResponse, Responder,
};
use models::users::User;
use mongodb::bson::{self, doc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AuthenticationRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(
    data: web::Data<crate::WebState>,
    auth_req: web::Json<AuthenticationRequest>,
) -> Result<HttpResponse, ResponseError> {
    let user = User {
        id: None,
        username: auth_req.username.clone(),
        password_hash: auth_req.password.clone(),
    };

    let user_document = bson::to_document(&user).expect("convert user to bson");

    let user = match User::collection(&data.db)
        .find_one(user_document, None)
        .await?
    {
        Some(user) => User::from_doc(user)?,
        _ => {
            return Err(ResponseError::new(
                "invalid credentials",
                StatusCode::UNAUTHORIZED,
            ))
        }
    };

    let token = AccessToken::generate_token(&data.db, user.id.expect("user id not found")).await?;

    return Ok(HttpResponse::Ok().json(token));
}

#[derive(Deserialize, Serialize)]
struct RegistrationRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(
    data: web::Data<crate::WebState>,
    register_req: web::Json<RegistrationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user =
        models::users::User::register(&data.db, &register_req.username, &register_req.password)
            .await?;

    return Ok(HttpResponse::Ok().json(user));
}

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lingo API Auth")
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/auth")
        .service(hello)
        .service(login)
        .service(register)
}
