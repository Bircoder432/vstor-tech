// src/frontend/api/mod.rs
pub mod projects;
pub mod skills;

use crate::frontend::errors::FrontendError;

const API_BASE: &str = "http://localhost:3000/api";

pub async fn fetch_api<T>(_endpoint: &str) -> Result<T, FrontendError>
where
    T: serde::de::DeserializeOwned + Default,
{
    // Пока заглушка - возвращаем пустые данные
    Ok(T::default())
}
