use crate::services::database::Database;
use models::Task;

// HINT: handlers stay thin — only delegate to the database service.
// The View layer should never call `Database` directly.

pub async fn list(db: &Database, user_id: &str) -> Result<Vec<Task>, sqlx::Error> {
    todo!("delegate to db.list_tasks(user_id)")
}

pub async fn create(
    db: &Database,
    user_id: &str,
    title: &str,
    description: &str,
    due_at: Option<&str>,
) -> Result<Task, sqlx::Error> {
    todo!("delegate to db.create_task(...)")
}

pub async fn complete(db: &Database, id: &str) -> Result<Option<Task>, sqlx::Error> {
    todo!("delegate to db.complete_task(id), then log the change")
}

pub async fn remove(db: &Database, id: &str) -> Result<(), sqlx::Error> {
    todo!("delegate to db.delete_task(id)")
}
