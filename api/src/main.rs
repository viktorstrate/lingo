#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use resolvers::create_schema;
use routes::graphql;

mod database;
mod resolvers;
mod routes;

pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let db_pool = database::establish_connection();

  let gql_schema = std::sync::Arc::new(create_schema());

  println!("Starting server on http://localhost:8080");

  HttpServer::new(move || {
    App::new()
      .data(gql_schema.clone())
      .data(db_pool.clone())
      .service(graphql::graphql)
      .service(graphql::graphiql)
    // .wrap(middleware::NormalizePath::default())
    // .service(routes::routes())
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
