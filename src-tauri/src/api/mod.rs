pub mod openai;
pub mod server;
pub mod state;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Memory error: {0}")]
    Memory(#[from] crate::memory::MemoryError),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Server error: {0}")]
    Server(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            error: None,
        }
    }
    
    pub fn error(data: T, error: impl Into<String>) -> Self {
        Self {
            data,
            error: Some(error.into()),
        }
    }
}
