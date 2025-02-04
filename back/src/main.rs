use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::{json, serde_json, Json};
use rocket::{fairing::AdHoc, Build, Rocket};
use test_rocket_back::{
    task::{PaginatedTasks, Pagination, Task, Todo},
    DbConn,
};

#[macro_use]
extern crate rocket;

/// # `run_migrations`
/// Runs database migrations on application startup.
/// This ensures the database schema is up to date before the application begins serving requests.
///
/// ## Arguments
/// * `rocket` - The Rocket instance being configured
///
/// ## Returns
/// The configured Rocket instance
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

/// # `root`
/// Handles GET requests to the root path ("/").
/// Serves as a simple health check endpoint.
///
/// ## Returns
/// A static string greeting message
#[get("/")]
fn root() -> &'static str {
    "Hello, Rocket!"
}

/// # `all_json`
/// Retrieves a paginated list of tasks.
///
/// ## Arguments
/// * `page` - Optional page number (defaults to 1)
/// * `per_page` - Optional number of items per page (defaults to 10)
/// * `conn` - Database connection
///
/// ## Returns
/// JSON response containing paginated tasks or an error message
#[get("/all?<page>&<per_page>")]
async fn all(
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

/// # CORS Configuration
/// Implements CORS (Cross-Origin Resource Sharing) headers for the application.
/// Allows requests from localhost:5173 during development.
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

/// # `all_options`
/// Handles OPTIONS requests for CORS preflight requests.
/// This endpoint is necessary for CORS to work properly with browsers.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

/// # `toggle`
/// Toggles the completion status of a task.
///
/// ## Arguments
/// * `id` - The ID of the task to toggle
/// * `conn` - Database connection
///
/// ## Returns
/// JSON response indicating success or failure of the operation
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

/// # `create`
/// Creates a new task.
///
/// ## Arguments
/// * `todo` - The task to create
/// * `conn` - Database connection
///
/// ## Returns
/// JSON response containing the created task or an error message
#[post("/", data = "<todo>")]
async fn create(todo: Json<Todo>, conn: DbConn) -> Json<serde_json::Value> {
    match Task::insert(todo.into_inner(), &conn).await {
        Ok(task) => Json(json!({
            "status": "success",
            "message": "Successfully created task",
            "task": task
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Failed to create task: {}", e)
        })),
    }
}

/// # `rocket`
/// Configures and launches the Rocket application.
/// Sets up database connection, runs migrations, configures CORS, and mounts routes.
///
/// ## Returns
/// The configured Rocket instance
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .attach(Cors)
        .mount("/", routes![root, all_options])
        .mount("/task", routes![all, toggle, create])
}
