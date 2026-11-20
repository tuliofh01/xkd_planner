use crate::services::database::Database;
use models::PtimerSession;

pub async fn start(
    db: &Database,
    user_id: &str,
    task_id: &str,
) -> Result<PtimerSession, sqlx::Error> {
    todo!("delegate to db.start_timer(user_id, task_id)")
}

pub async fn stop(db: &Database, id: &str) -> Result<Option<PtimerSession>, sqlx::Error> {
    todo!("delegate to db.stop_timer(id)")
}

pub async fn list(db: &Database, user_id: &str) -> Result<Vec<PtimerSession>, sqlx::Error> {
    todo!("delegate to db.list_timers(user_id)")
}
