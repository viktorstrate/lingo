use actix_web::{App, HttpServer};

mod database;
mod models;
mod routes;

pub struct WebState {
    db: mongodb::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database::get_database().await.expect("get database");

    println!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .data(WebState { db: db.clone() })
            // .wrap(middleware::NormalizePath::default())
            .service(routes::routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
