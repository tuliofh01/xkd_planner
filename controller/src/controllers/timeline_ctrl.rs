use crate::services::database::Database;
use models::TimelineEntry;

pub async fn list(db: &Database, user_id: &str) -> Result<Vec<TimelineEntry>, sqlx::Error> {
    todo!("delegate to db.list_timeline(user_id)")
}

pub async fn add(
    db: &Database,
    user_id: &str,
    task_id: &str,
    starts_at: &str,
    ends_at: Option<&str>,
) -> Result<TimelineEntry, sqlx::Error> {
    todo!("delegate to db.add_timeline_entry(...)")
}
