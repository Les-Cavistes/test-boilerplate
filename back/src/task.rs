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

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tasks)]
pub struct Task {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, rocket::FromForm)]
pub struct Todo {
    pub description: String,
}

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
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Task>> {
        conn.run(|c| tasks::table.order(tasks::id.desc()).load::<Task>(c))
            .await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(todo: Todo, conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| {
            let t = Task {
                id: None,
                description: todo.description,
                completed: false,
            };
            diesel::insert_into(tasks::table).values(&t).execute(c)
        })
        .await
    }

    /// Returns the number of affected rows: 1.
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

    /// Returns the number of affected rows: 1.
    pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(tasks::table)
                .filter(tasks::id.eq(id))
                .execute(c)
        })
        .await
    }

    /// Returns the number of affected rows.
    #[cfg(test)]
    pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
        conn.run(|c| diesel::delete(tasks::table).execute(c)).await
    }

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
}
