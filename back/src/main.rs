use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, Build, Rocket};
use test_rocket_back::{task::Task, DbConn};

#[macro_use]
extern crate rocket;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

#[get("/")]
fn root() -> &'static str {
    "Hello, Rocket!"
}

#[get("/all")]
async fn all_json(conn: DbConn) -> Result<Json<Vec<Task>>, String> {
    Task::all(&conn).await.map(Json).map_err(|e| e.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", routes![root, all_json])
}
