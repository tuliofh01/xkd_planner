use serde::{Deserialize, Serialize};

pub const STATUS_PENDING: &str = "pending";
pub const STATUS_COMPLETED: &str = "completed";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Task record for a user's daily plan.
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub due_at: Option<String>,
    pub status: String,
    pub created_at: String,
}

impl Task {
    // HINT: start every task with STATUS_PENDING.
    /// Creates a pending task.
    pub fn new(
        id: String,
        user_id: String,
        title: String,
        description: String,
        due_at: Option<String>,
        created_at: String,
    ) -> Self {
        todo!("assign all fields; status = STATUS_PENDING.to_string()")
    }

    // HINT: compare self.status with STATUS_COMPLETED.
    /// Returns true when the task status is completed.
    pub fn is_completed(&self) -> bool {
        todo!("return self.status == STATUS_COMPLETED")
    }

    // HINT: mutate self.status.
    /// Marks the task as completed.
    pub fn complete(&mut self) {
        todo!("set self.status to STATUS_COMPLETED.to_string()")
    }
}
