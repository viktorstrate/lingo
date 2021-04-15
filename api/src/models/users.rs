use super::ResponseError;
use crate::schema::users;
use diesel::pg::PgConnection;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::diesel::prelude::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize, GraphQLObject)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub username: String,
  #[graphql(skip)]
  pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub password_hash: &'a str,
}

pub fn register_user(
  conn: &PgConnection,
  username: &str,
  password: &str,
) -> Result<User, ResponseError> {
  let new_user = NewUser {
    username: username,
    password_hash: password,
  };

  let user = diesel::insert_into(users::table)
    .values(&new_user)
    .get_result::<User>(conn)?;

  return Ok(user);
}

pub fn authorize_user(conn: &PgConnection, username_input: &str) -> Result<User, ResponseError> {
  use crate::schema::users::dsl::*;

  let result = users
    .filter(username.eq(username_input))
    .limit(1)
    .load::<User>(conn)?;

  match result.into_iter().nth(0) {
    Some(user) => Ok(user),
    None => Err(ResponseError::new(
      "User not found",
      actix_web::http::StatusCode::NOT_FOUND,
    )),
  }
}
