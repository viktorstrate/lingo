use diesel::{
  prelude::*,
  r2d2::{ConnectionManager, Pool},
};
use std::env;

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DatabasePool {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let conn_manager = ConnectionManager::<PgConnection>::new(database_url);

  Pool::builder()
    .max_size(15)
    .build(conn_manager)
    .expect("Failed to build connection pool")
}
