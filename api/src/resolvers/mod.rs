use std::sync::Arc;

use crate::database::DatabasePool;
use juniper::EmptySubscription;
use juniper::RootNode;

use self::{mutation::MutationRoot, query::QueryRoot};

pub mod mutation;
pub mod query;

pub struct Context {
  pub pool: Arc<DatabasePool>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
