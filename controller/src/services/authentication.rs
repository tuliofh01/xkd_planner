use models::User;

use crate::services::database::Database;
use crate::services::AppError;

#[derive(Clone)]
/// Authentication service for user registration and login.
pub struct Authentication {
    db: Database,
}

impl Authentication {
    /// Creates an authentication service backed by a database.
    pub fn new(db: Database) -> Self {
        todo!("store db")
    }

    // HINT: validate input, reject duplicates, hash the password, INSERT, return redacted user.
    /// Registers a new user and returns a redacted user record.
    pub async fn register(&self, username: &str, password: &str) -> Result<User, AppError> {
        todo!("register a new user")
    }

    // HINT: look up the user, verify the hash, attach tasks, return redacted user.
    /// Authenticates a user and returns a redacted user record.
    pub async fn login(&self, username: &str, password: &str) -> Result<User, AppError> {
        todo!("authenticate an existing user")
    }
}
