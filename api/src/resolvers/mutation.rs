use crate::models::users::{self, User};

use super::Context;
use juniper::{graphql_object, FieldResult};

pub struct MutationRoot;

#[graphql_object(
  context = Context,
)]
impl MutationRoot {
  fn user(context: &Context, username: String) -> FieldResult<User> {
    // Get a db connection.
    let connection = context.pool.get()?;
    // Execute a db query.
    // Note the use of `?` to propagate errors.
    let user = users::authorize_user(&connection, &username)?;
    // Return the result.
    Ok(user)
  }
}
