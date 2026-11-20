use serde::{Deserialize, Serialize};

use super::task_record::Task;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User account record.
///
/// `password` should contain a hash, never a plaintext password.
pub struct User {
    pub id: String,
    pub username: String,
    // HINT: store a password hash here, never the raw password.
    pub password: String,
    pub status: bool,
    pub tasks: Vec<Task>,
}

impl User {
    // HINT: build a new active user with an empty task list.
    /// Creates a new active user with an empty task list.
    pub fn new(id: String, username: String, password: String) -> Self {
        todo!("set status = true and tasks = Vec::new()")
    }

    // HINT: filter self.tasks with Iterator::filter.
    /// Returns tasks that are not completed.
    pub fn active_tasks(&self) -> Vec<&Task> {
        todo!("return tasks whose status is not STATUS_COMPLETED")
    }

    // HINT: the mirror of active_tasks().
    /// Returns completed tasks.
    pub fn completed_tasks(&self) -> Vec<&Task> {
        todo!("return tasks whose status is STATUS_COMPLETED")
    }

    // HINT: clone self, then clear the password so the hash never leaves the crate.
    /// Returns a copy safe to expose to the UI (password removed).
    pub fn redacted(&self) -> Self {
        todo!("return a clone with an empty password")
    }
}
