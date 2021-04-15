use super::{users, ResponseError};
use crate::schema::access_tokens;
use actix_web::{http::header::AUTHORIZATION, web::Data};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::{future::Future, pin::Pin};

#[derive(Queryable, Identifiable)]
#[table_name = "access_tokens"]
pub struct AccessToken {
    pub id: i32,
    pub user_id: i32,
    pub expire_at: chrono::NaiveDateTime,
    pub token: String,
}

// impl AccessToken {
//     pub async fn generate_token(
//         db: &mongodb::Database,
//         user_id: ObjectId,
//     ) -> Result<AccessToken, ResponseError> {
//         let access_token = AccessToken {
//             id: None,
//             user_id: user_id,
//             expire_at: DateTime::from(chrono::Utc::now() + chrono::Duration::days(30)),
//             token: Uuid::new_v4().to_string(),
//         };

//         AccessToken::collection(&db)
//             .insert_one(access_token.to_doc()?, None)
//             .await?;

//         return Ok(access_token);
//     }

//     pub async fn get_user(&self, db: &mongodb::Database) -> Result<User, ResponseError> {
//         match User::collection(&db)
//             .find_one(doc! { "_id": &self.user_id }, None)
//             .await
//         {
//             Ok(Some(doc)) => User::from_doc(doc),
//             Ok(None) => Err(ResponseError::new(
//                 "user not found",
//                 StatusCode::UNAUTHORIZED,
//             )),
//             Err(err) => Err(ResponseError::error(Some(&err), "error")),
//         }
//     }
// }

impl actix_web::FromRequest for AccessToken {
    type Error = ResponseError;
    // type Future = Ready<Result<Self, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let header_value = req.headers().get(AUTHORIZATION).map(|val| val.clone());

        let conn = req
            .app_data::<Data<Pool<ConnectionManager<PgConnection>>>>()
            .unwrap()
            .get()
            .unwrap();

        let user = actix_web::web::block::<_, _, ()>(move || {
            Ok(users::register_user(&conn, "user", "pass"))
        });

        Box::pin(async move {
            let user_wait = user.await;

            return Ok(AccessToken {
                expire_at: chrono::NaiveDateTime::from_timestamp(0, 0),
                id: 123,
                token: "the token here".to_owned(),
                user_id: 222,
            });

            // let auth_header = match header_value {
            //     Some(header) => match header.to_str() {
            //         Ok(str) => str.to_owned(),
            //         Err(err) => {
            //             return Err(ResponseError::error(
            //                 Some(&err),
            //                 "invalid AUTHORIZATION header",
            //             ))
            //         }
            //     },
            //     None => {
            //         return Err(ResponseError::new(
            //             "missing access token",
            //             StatusCode::UNAUTHORIZED,
            //         ))
            //     }
            // };

            // if !auth_header.to_lowercase().starts_with("bearer ") {
            //     return Err(ResponseError::new(
            //         "AUTHORIZATION header is invalid bearer",
            //         StatusCode::BAD_REQUEST,
            //     ));
            // }

            // let token_value = match auth_header.split(' ').nth(1) {
            //     Some(token) => token,
            //     _ => {
            //         return Err(ResponseError::new(
            //             "AUTHORIZATION header is invalid bearer",
            //             StatusCode::BAD_REQUEST,
            //         ))
            //     }
            // };

            // let token = match AccessToken::collection(&db)
            //     .find_one(doc! { "token": token_value }, None)
            //     .await
            // {
            //     Ok(Some(token)) => AccessToken::from_doc(token),
            //     Ok(None) => Err(ResponseError::new(
            //         "invalid token",
            //         StatusCode::UNAUTHORIZED,
            //     )),
            //     Err(err) => Err(ResponseError::error(
            //         Some(&err),
            //         "failed to convert access token",
            //     )),
            // };

            // return token;
        })
    }
}
