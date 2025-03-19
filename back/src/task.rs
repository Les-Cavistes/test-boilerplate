use crate::pagination::{Paginate, PaginationResult};
use crate::schema::tasks;
use diesel::query_dsl::QueryDsl;
use diesel::{self, prelude::*};
use rocket::serde::Serialize;

use crate::DbConn;

#[derive(Insertable, Debug)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub description: String,
    pub completed: bool,
}

/// # Task
/// Represents a task in the system.
/// This struct maps directly to the database 'tasks' table.
#[derive(Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    #[serde(skip_deserializing)]
    pub id: i32, // Non-optional for querying
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

impl Task {
    /// # `all`
    /// Retrieves all tasks from the database, ordered by ID in descending order.
    ///
    /// ## Arguments
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// A Result containing a vector of all tasks
    ///
    /// ## Errors
    /// Returns a `QueryResult` error if the database operation fails
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
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the insert operation fails
    /// * Returns a `QueryResult` error if the task cannot be retrieved after
    pub async fn insert(todo: Todo, conn: &DbConn) -> QueryResult<Task> {
        conn.run(|c| {
            let new_task = NewTask {
                description: todo.description,
                completed: false,
            };
            diesel::insert_into(tasks::table)
                .values(&new_task)
                .get_result(c)
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
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the task with given ID is not found
    /// * Returns a `QueryResult` error if the update operation fails
    pub async fn toggle_with_id(id: i32, conn: &DbConn) -> QueryResult<Task> {
        conn.run(move |c| {
            let task = tasks::table
                .select(Task::as_select())
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
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the task with given ID is not found
    /// * Returns a `QueryResult` error if the delete operation fails
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
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the task with given ID is not found
    /// * Returns a `QueryResult` error if the delete operation fails
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(tasks::table).execute(c)).await
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
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the task with given ID is not found
    /// * Returns a `QueryResult` error if the update operation fails
    pub async fn update_description(
        id: i32,
        new_description: String,
        conn: &DbConn,
    ) -> QueryResult<Task> {
        conn.run(move |c| {
            diesel::update(tasks::table.filter(tasks::id.eq(id)))
                .set(tasks::description.eq(new_description))
                .execute(c)?;

            // Fetch and return the updated task
            tasks::table.filter(tasks::id.eq(id)).get_result::<Task>(c)
        })
        .await
    }

    /// # `all_paginated`
    /// Retrieves all tasks from the database, paginated.
    /// The tasks are ordered by ID in descending order.
    ///
    /// ## Arguments
    /// * `page` - The page number to retrieve
    /// * `per_page` - The number of items per page
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// A Result containing a `PaginationResult` of tasks
    ///
    /// ## Errors
    /// * Returns a `QueryResult` error if the database operation fails
    /// * Returns a `QueryResult` error if pagination calculation fails
    pub async fn all_paginated(
        page: i64,
        per_page: i64,
        conn: &DbConn,
    ) -> QueryResult<PaginationResult<Task>> {
        conn.run(move |c| {
            tasks::table
                .select(Task::as_select())
                .order(tasks::id.desc())
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages(c)
        })
        .await
    }
}
