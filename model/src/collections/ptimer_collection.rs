use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PtimerSession {
    pub id: String,
    pub user_id: String,
    pub task_id: String,
    pub started_at: String,
    pub ended_at: Option<String>,
}

impl PtimerSession {
    // HINT: a fresh session has ended_at = None.
    pub fn new(id: String, user_id: String, task_id: String, started_at: String) -> Self {
        todo!("assign fields; ended_at = None")
    }

    pub fn is_running(&self) -> bool {
        todo!("return self.ended_at.is_none()")
    }

    pub fn stop(&mut self, ended_at: String) {
        todo!("set self.ended_at = Some(ended_at)")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PtimerCollection {
    pub id: String,
    pub user_id: String,
    pub sessions: Vec<PtimerSession>,
}

impl PtimerCollection {
    pub fn new(id: String, user_id: String) -> Self {
        todo!("initialize sessions as Vec::new()")
    }

    // HINT: filter with PtimerSession::is_running.
    pub fn running(&self) -> Vec<&PtimerSession> {
        todo!("return only sessions that are still running")
    }
}
