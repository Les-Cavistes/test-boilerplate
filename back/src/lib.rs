pub mod task;

#[rocket_sync_db_pools::database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);
