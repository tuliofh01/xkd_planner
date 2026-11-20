use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Activity/log record, optionally linked to a task.
pub struct SlogRecord {
    pub id: String,
    pub user_id: String,
    pub task_id: Option<String>,
    pub message: String,
    pub created_at: String,
}

impl SlogRecord {
    // HINT: a log may be global (task_id = None) or tied to a task.
    /// Creates a log record.
    pub fn new(
        id: String,
        user_id: String,
        task_id: Option<String>,
        message: String,
        created_at: String,
    ) -> Self {
        todo!("assign all fields directly")
    }
}
