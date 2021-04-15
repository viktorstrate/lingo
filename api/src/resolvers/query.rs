use juniper::FieldResult;
use juniper::RootNode;

use crate::models::users::{self, User};

use super::Context;
use juniper::{
  graphql_object, EmptySubscription, GraphQLEnum, GraphQLInputObject, GraphQLObject, ScalarValue,
};

pub struct QueryRoot;

#[graphql_object(
  context = Context,
)]
impl QueryRoot {
  fn apiVersion() -> &str {
    "1.0"
  }
}
