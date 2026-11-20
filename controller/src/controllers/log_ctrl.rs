use crate::services::database::Database;
use models::SlogRecord;

pub async fn list(db: &Database, user_id: &str) -> Result<Vec<SlogRecord>, sqlx::Error> {
    todo!("delegate to db.list_logs(user_id)")
}

// HINT: called by other handlers when something notable happens.
pub async fn write(
    db: &Database,
    user_id: &str,
    task_id: Option<&str>,
    message: &str,
) -> Result<SlogRecord, sqlx::Error> {
    todo!("delegate to db.create_log(...)")
}
