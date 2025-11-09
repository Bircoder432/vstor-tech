use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),

    #[error("API error: {0}")]
    Api(#[from] ApiError),

    #[error("External service error: {0}")]
    ExternalService(String),
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Query failed: {0}")]
    Query(String),

    #[error("Migration failed: {0}")]
    Migration(String),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Access denied")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    BadRequest(String),
}
