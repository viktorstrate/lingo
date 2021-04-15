use std::sync::Arc;

use actix_web::{
  get, post,
  web::{self},
  HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::{
  database::DatabasePool,
  models::ResponseError,
  resolvers::{Context, Schema},
};

#[post("/graphql")]
pub async fn graphql(
  st: web::Data<Arc<Schema>>,
  pool: web::Data<DatabasePool>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, ResponseError> {
  let ctx = Context {
    pool: pool.into_inner(),
  };

  let result = web::block(move || {
    let res = data.execute_sync(&st, &ctx);
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .await?;

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(result),
  )
}

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
  let html = graphiql_source("/graphql", None);
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}
