use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyCollection {
    pub id: String,
    pub user_id: String,
    pub day: String,
    pub task_ids: Vec<String>,
}

impl DailyCollection {
    // HINT: begin with an empty task_ids vector.
    pub fn new(id: String, user_id: String, day: String) -> Self {
        todo!("initialize task_ids as Vec::new()")
    }

    // HINT: avoid duplicates with Vec::contains before pushing.
    pub fn add_task(&mut self, task_id: String) {
        todo!("push only when task_id is not already present")
    }

    // HINT: Vec::retain is handy here.
    pub fn remove_task(&mut self, task_id: &str) {
        todo!("keep only ids that differ from task_id")
    }

    pub fn contains(&self, task_id: &str) -> bool {
        todo!("return whether task_ids contains task_id")
    }
}
