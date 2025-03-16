use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

pub fn setup_test_db() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/todo_test".to_string());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool")
}