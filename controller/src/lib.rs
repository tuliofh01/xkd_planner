//! Controller and service crate for xkdPlanner.
//!
//! Exposes handlers, persistence/services, and shared application state for the Tauri view.

// HINT: skeleton bodies are `todo!()`, so unused-parameter/dead-code warnings
// are expected. Remove this attribute once the implementation is complete.
#![allow(unused_variables, dead_code)]

pub mod controllers;
pub mod services;

pub use models::{
    DailyCollection, PtimerCollection, PtimerSession, SlogRecord, Task, TimelineCollection,
    TimelineEntry, User,
};
pub use services::authentication::Authentication;
pub use services::database::Database;

/// Shared application state, handed to Tauri commands through `tauri::State`.
#[derive(Clone)]
/// Shared application services managed by Tauri state.

pub struct AppServices {
    pub db: Database,
    pub auth: Authentication,
}

impl AppServices {
    // HINT: Database::connect(url), db.init(), then Authentication::new(db.clone()).
    /// Connects to the database, initializes schema, and builds services.

    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        todo!("connect and initialise the database")
    }

    /// Builds services from an existing database handle.

    pub fn from(db: Database) -> Self {
        todo!("build Authentication from a clone of db")
    }
}
