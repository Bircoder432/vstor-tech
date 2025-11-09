use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FrontendError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Rendering error: {0}")]
    Render(String),

    #[error("State error: {0}")]
    State(String),

    #[error("User input error: {0}")]
    Validation(String),
}
