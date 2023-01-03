use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool() -> DbPool {
    let database_url: String = crate::app::config::get("database_url");

    println!("Connecting to database {}...", database_url);
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
}
