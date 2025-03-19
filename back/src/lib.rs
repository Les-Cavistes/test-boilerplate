/// # Database Connection Pool
/// Provides a PostgreSQL connection pool using rocket_sync_db_pools.
/// This struct is used throughout the application to interact with the database.
#[rocket_sync_db_pools::database("postgres_database")]
pub struct DbConn(diesel::PgConnection);

pub mod pagination;
pub mod schema;
pub mod task;

pub const MAX_PER_PAGE: i64 = 100; // Prevent excessive page sizes
pub const DEFAULT_PER_PAGE: i64 = 10;
pub const DEFAULT_PAGE: i64 = 1;
