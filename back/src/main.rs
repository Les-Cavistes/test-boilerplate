use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::{json, serde_json, Json};
use rocket::{fairing::AdHoc, Build, Rocket};
use test_rocket_back::task::{PaginatedTasks, Pagination};
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

#[get("/all?<page>&<per_page>")]
async fn all_json(
    page: Option<i64>,
    per_page: Option<i64>,
    conn: DbConn,
) -> Result<Json<PaginatedTasks>, String> {
    let pagination = Pagination {
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(10),
    };
    Task::paginated(pagination, &conn)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:5173",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .attach(Cors)
        .mount("/", routes![root, all_json, all_options, toggle])
}

#[put("/<id>")]
async fn toggle(id: i32, conn: DbConn) -> Json<serde_json::Value> {
    match Task::toggle_with_id(id, &conn).await {
        Ok(task) => Json(json!({
            "status": "success",
            "message": format!("Successfully toggled task {}", id),
            "completed": task.completed
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Failed to toggle task: {}", e)
        })),
    }
}
