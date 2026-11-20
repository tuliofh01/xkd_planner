use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: String,
    pub user_id: String,
    pub task_id: String,
    pub starts_at: String,
    pub ends_at: Option<String>,
}

impl TimelineEntry {
    // HINT: open-ended entries use ends_at = None.
    pub fn new(
        id: String,
        user_id: String,
        task_id: String,
        starts_at: String,
        ends_at: Option<String>,
    ) -> Self {
        todo!("assign all fields directly")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineCollection {
    pub id: String,
    pub user_id: String,
    pub entries: Vec<TimelineEntry>,
}

impl TimelineCollection {
    pub fn new(id: String, user_id: String) -> Self {
        todo!("initialize entries as Vec::new()")
    }

    pub fn add(&mut self, entry: TimelineEntry) {
        todo!("push the entry")
    }

    // HINT: filter entries by their task_id field.
    pub fn for_task(&self, task_id: &str) -> Vec<&TimelineEntry> {
        todo!("return entries belonging to task_id")
    }
}
