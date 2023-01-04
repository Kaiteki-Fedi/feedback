use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_connection_pool() -> DbPool {
    if cfg!(test) {
        println!("Connecting to in-memory database...");
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool.");

        let mut con = pool.get().unwrap();

        let test: &mut SqliteConnection = &mut con;

        println!("Running migrations...");

        test.run_pending_migrations(MIGRATIONS).unwrap();

        pool
    } else {
        let database_url: String = crate::app::config::get("database_url");

        println!("Connecting to database {}...", database_url);
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool.")
    }
}
