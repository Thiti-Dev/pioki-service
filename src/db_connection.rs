use r2d2::Pool;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::env;
use log;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    log::info!("Connecting to the database.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .build(manager)
        .expect("Failed to create connection Pool.")
}