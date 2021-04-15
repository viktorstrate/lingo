// use crate::models::{self, users::AccessToken, Model, ResponseError};
use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
// use models::users::User;
use serde::{Deserialize, Serialize};

use crate::{
    database::DatabasePool,
    models::{self, ResponseError},
};

#[derive(Deserialize, Serialize)]
struct AuthenticationRequest {
    username: String,
    password: String,
}

// #[post("/login")]
// async fn login(
//     data: web::Data<crate::WebState>,
//     auth_req: web::Json<AuthenticationRequest>,
// ) -> Result<HttpResponse, ResponseError> {
//     let user = User {
//         id: None,
//         username: auth_req.username.clone(),
//         password_hash: auth_req.password.clone(),
//     };

//     let user_document = bson::to_document(&user).expect("convert user to bson");

//     let user = match User::collection(&data.db)
//         .find_one(user_document, None)
//         .await?
//     {
//         Some(user) => User::from_doc(user)?,
//         _ => {
//             return Err(ResponseError::new(
//                 "invalid credentials",
//                 StatusCode::UNAUTHORIZED,
//             ))
//         }
//     };

//     let token = AccessToken::generate_token(&data.db, user.id.expect("user id not found")).await?;

//     return Ok(HttpResponse::Ok().json(token));
// }

#[derive(Deserialize, Serialize)]
struct RegistrationRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(
    pool: Data<DatabasePool>,
    register_req: web::Json<RegistrationRequest>,
) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get()?;

    let user = web::block::<_, _, ()>(move || {
        Ok(models::users::register_user(
            &conn,
            &register_req.username,
            &register_req.password,
        ))
    })
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
        // .service(login)
        .service(register)
}
