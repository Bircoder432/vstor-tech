use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharedError {
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },
}
