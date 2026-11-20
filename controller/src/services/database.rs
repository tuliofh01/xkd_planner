//! SQLite persistence layer.
//!
//! Provides CRUD operations for users, tasks, logs, timers, and timeline entries.

use models::{PtimerSession, SlogRecord, Task, TimelineEntry, User};
use sqlx::sqlite::{SqlitePool, SqliteRow};

/// Thin wrapper around a `SqlitePool`. All persistence lives here.
#[derive(Debug, Clone)]
/// Database handle wrapping a SQLite connection pool.
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    // HINT: `SqlitePool::connect(url).await?`
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        todo!("open the pool")
    }

    // HINT: use SqlitePoolOptions with max_connections(1) and "sqlite::memory:".
    pub async fn in_memory() -> Result<Self, sqlx::Error> {
        todo!("open a single-connection in-memory pool for tests")
    }

    // HINT: run CREATE TABLE IF NOT EXISTS for users, tasks, slog_records,
    // ptimer_sessions and timeline_entries.
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        todo!("create the schema")
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        todo!("INSERT INTO users, storing user.password as password_hash")
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        todo!("SELECT ... WHERE username = ? then map_user")
    }

    pub async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        todo!("SELECT all users then map_user")
    }

    pub async fn create_task(
        &self,
        user_id: &str,
        title: &str,
        description: &str,
        due_at: Option<&str>,
    ) -> Result<Task, sqlx::Error> {
        todo!("generate a uuid + timestamp, INSERT, return the Task")
    }

    pub async fn get_task(&self, id: &str) -> Result<Option<Task>, sqlx::Error> {
        todo!("SELECT one task then map_task")
    }

    pub async fn list_tasks(&self, user_id: &str) -> Result<Vec<Task>, sqlx::Error> {
        todo!("SELECT tasks for user_id ordered by created_at")
    }

    pub async fn complete_task(&self, id: &str) -> Result<Option<Task>, sqlx::Error> {
        todo!("UPDATE status to models::STATUS_COMPLETED, then get_task")
    }

    pub async fn delete_task(&self, id: &str) -> Result<(), sqlx::Error> {
        todo!("DELETE FROM tasks WHERE id = ?")
    }

    pub async fn create_log(
        &self,
        user_id: &str,
        task_id: Option<&str>,
        message: &str,
    ) -> Result<SlogRecord, sqlx::Error> {
        todo!("generate an id + timestamp, INSERT, return the SlogRecord")
    }

    pub async fn list_logs(&self, user_id: &str) -> Result<Vec<SlogRecord>, sqlx::Error> {
        todo!("SELECT logs for user_id then map_log")
    }

    pub async fn start_timer(&self, user_id: &str, task_id: &str) -> Result<PtimerSession, sqlx::Error> {
        todo!("INSERT a session with ended_at = NULL")
    }

    pub async fn stop_timer(&self, id: &str) -> Result<Option<PtimerSession>, sqlx::Error> {
        todo!("SET ended_at to now, then get_timer")
    }

    pub async fn get_timer(&self, id: &str) -> Result<Option<PtimerSession>, sqlx::Error> {
        todo!("SELECT one session then map_timer")
    }

    pub async fn list_timers(&self, user_id: &str) -> Result<Vec<PtimerSession>, sqlx::Error> {
        todo!("SELECT sessions for user_id")
    }

    pub async fn add_timeline_entry(
        &self,
        user_id: &str,
        task_id: &str,
        starts_at: &str,
        ends_at: Option<&str>,
    ) -> Result<TimelineEntry, sqlx::Error> {
        todo!("INSERT a timeline entry, return it")
    }

    pub async fn list_timeline(&self, user_id: &str) -> Result<Vec<TimelineEntry>, sqlx::Error> {
        todo!("SELECT entries for user_id")
    }
}

// HINT: each mapper reads columns via `row.try_get("column_name")?`.
fn map_user(row: &SqliteRow) -> Result<User, sqlx::Error> {
    todo!("password <- 'password_hash'; status <- integer != 0; tasks <- Vec::new()")
}

fn map_task(row: &SqliteRow) -> Result<Task, sqlx::Error> {
    todo!("read all seven task columns")
}

fn map_log(row: &SqliteRow) -> Result<SlogRecord, sqlx::Error> {
    todo!("read all five log columns")
}

fn map_timer(row: &SqliteRow) -> Result<PtimerSession, sqlx::Error> {
    todo!("read all five session columns")
}

fn map_entry(row: &SqliteRow) -> Result<TimelineEntry, sqlx::Error> {
    todo!("read all five timeline columns")
}
