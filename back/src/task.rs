use diesel::{self, prelude::*};
use rocket::serde::Serialize;

mod schema {

    diesel::table! {
        tasks {
            id -> Nullable<Integer>,
            description -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::tasks;

use crate::DbConn;

/// # Task
/// Represents a task in the system.
/// This struct maps directly to the database 'tasks' table.
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tasks)]
pub struct Task {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool,
}

/// # Todo
/// Represents the input format for creating a new task.
/// Contains only the description as the completed status is set to false by default.
#[derive(Debug, rocket::FromForm, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub description: String,
}

/// # Pagination
/// Handles pagination parameters for task listing.
#[derive(Debug)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

/// # PaginatedTasks
/// Contains a page of tasks along with pagination metadata.
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginatedTasks {
    pub tasks: Vec<Task>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl Task {
    /// # `all`
    /// Retrieves all tasks from the database, ordered by ID in descending order.
    ///
    /// ## Arguments
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// A Result containing a vector of all tasks
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Task>> {
        conn.run(|c| tasks::table.order(tasks::id.desc()).load::<Task>(c))
            .await
    }

    /// # `insert`
    /// Creates a new task in the database.
    ///
    /// ## Arguments
    /// * `todo` - The todo item to insert
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// The newly created task
    pub async fn insert(todo: Todo, conn: &DbConn) -> QueryResult<Task> {
        conn.run(|c| {
            let t = Task {
                id: None,
                description: todo.description,
                completed: false,
            };
            diesel::insert_into(tasks::table).values(&t).execute(c)?;

            // Fetch and return the inserted task
            tasks::table.order(tasks::id.desc()).first(c)
        })
        .await
    }

    /// # `toggle_with_id`
    /// Toggles the completion status of a task.
    ///
    /// ## Arguments
    /// * `id` - The ID of the task to toggle
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// The updated task
    pub async fn toggle_with_id(id: i32, conn: &DbConn) -> QueryResult<Task> {
        conn.run(move |c| {
            let task = tasks::table
                .filter(tasks::id.eq(id))
                .get_result::<Task>(c)?;
            let new_status = !task.completed;
            diesel::update(tasks::table.filter(tasks::id.eq(id)))
                .set(tasks::completed.eq(new_status))
                .execute(c)?;

            // Fetch and return the updated task
            tasks::table.filter(tasks::id.eq(id)).get_result::<Task>(c)
        })
        .await
    }

    /// # `delete_with_id`
    /// Deletes a task from the database.
    ///
    /// ## Arguments
    /// * `id` - The ID of the task to delete
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// The number of affected rows (should be 1)
    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(tasks::table)
                .filter(tasks::id.eq(id))
                .execute(c)
        })
        .await
    }

    /// # `delete_all`
    /// Deletes all tasks from the database.
    /// This function is only available in test builds.
    ///
    /// ## Arguments
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// The number of deleted tasks
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(tasks::table).execute(c)).await
    }

    /// # `paginated`
    /// Retrieves a paginated list of tasks.
    ///
    /// ## Arguments
    /// * `pagination` - Pagination parameters
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// A paginated result containing tasks and metadata
    pub async fn paginated(pagination: Pagination, conn: &DbConn) -> QueryResult<PaginatedTasks> {
        conn.run(move |c| {
            // Get total count
            let total = tasks::table.count().get_result::<i64>(c)?;

            // Calculate offset
            let offset = (pagination.page - 1) * pagination.per_page;

            // Get paginated tasks
            let tasks = tasks::table
                .order(tasks::id.desc())
                .limit(pagination.per_page)
                .offset(offset)
                .load::<Task>(c)?;

            // Calculate total pages
            let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

            Ok(PaginatedTasks {
                tasks,
                total,
                page: pagination.page,
                per_page: pagination.per_page,
                total_pages,
            })
        })
        .await
    }

    /// # `update_description`
    /// Updates a task's description.
    ///
    /// ## Arguments
    /// * `id` - The ID of the task to update
    /// * `new_description` - The new description for the task
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// The updated task
    pub async fn update_description(id: i32, new_description: String, conn: &DbConn) -> QueryResult<Task> {
        conn.run(move |c| {
            diesel::update(tasks::table.filter(tasks::id.eq(id)))
                .set(tasks::description.eq(new_description))
                .execute(c)?;

            // Fetch and return the updated task
            tasks::table.filter(tasks::id.eq(id)).get_result::<Task>(c)
        })
        .await
    }
}
