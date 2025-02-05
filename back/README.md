# Rust Rocket Task Management API

A REST API built with Rocket framework for managing tasks, featuring SQLite database integration and pagination support.

## Project Structure

```
back/
├── migrations/               # Database migration files
│   └── 20250204141630_create_tasks_table/
│       ├── up.sql          # Database setup
│       └── down.sql        # Database teardown
└── src/
    ├── lib.rs              # Core library definitions
    ├── main.rs             # Application entry point and route handlers
    └── task.rs             # Task model and business logic
```

## Key Components

### Database Connection (DbConn)

The project uses `rocket_sync_db_pools` for database connectivity:

```rust
#[rocket_sync_db_pools::database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);
```

This provides:
- Automatic connection pooling
- Async/await support for database operations
- Integration with Rocket's lifecycle management

### Task Management

Tasks are managed through the `Task` struct with the following features:
- CRUD operations
- Status toggling
- Pagination support
- Automatic ID generation

### Pagination System

The API implements a robust pagination system using:

```rust
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

pub struct PaginatedTasks {
    pub tasks: Vec<Task>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
```

## Rocket Framework Features

### Error Handling

The API uses Diesel's `QueryResult` for database operations:
- Type-safe error handling
- Automatic conversion to JSON responses
- Consistent error messaging

### Route Handlers

Rocket's declarative routing system:
```rust
#[get("/all?<page>&<per_page>")]
#[post("/", data = "<todo>")]
#[patch("/<id>/toggle")]
#[delete("/<id>")]
```

### CORS Support

Built-in CORS configuration for development:
- Configurable origins
- Automatic headers handling
- OPTIONS request support

## API Endpoints

- `GET /task/all` - List tasks (with pagination)
- `POST /task` - Create new task
- `PATCH /task/<id>/toggle` - Toggle task completion
- `DELETE /task/<id>` - Delete task
- `PUT /task/<id>` - Update task description

## Database Features

### Migrations

Automatic migration system using Diesel:
```rust
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
```

### Models

Type-safe database interactions using Diesel's ORM:
```rust
#[derive(Serialize, Queryable, Insertable)]
pub struct Task {
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool,
}
```

## Getting Started

1. Install dependencies:
   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   ```

2. Set up the database:
   ```bash
   diesel setup
   diesel migration run
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## Technical Details

- **Framework**: Rocket 0.5+
- **Database**: SQLite with Diesel ORM
- **Authentication**: Not implemented (can be added as needed)
- **API Format**: JSON
- **Async Support**: Full async/await support for database operations
```
