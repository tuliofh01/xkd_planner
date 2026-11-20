pub mod authentication;
pub mod database;
pub mod encryption;

// HINT: add new variants as your services grow (e.g. NotFound).
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    InvalidCredentials,
    UserExists,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "database error: {error}"),
            Self::InvalidCredentials => write!(formatter, "invalid username or password"),
            Self::UserExists => write!(formatter, "username already exists"),
        }
    }
}

impl std::error::Error for AppError {}
