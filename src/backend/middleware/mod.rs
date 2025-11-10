// backend/middleware/mod.rs
pub mod auth;
pub mod logging;
pub mod protect_write;

// Реэкспорты для удобства
pub use auth::auth_middleware;
pub use logging::logging_middleware;
pub use protect_write::protect_write_operations;
