use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub status: String,
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(code: u16, data: T, message: impl Into<String>) -> Self {
        Self {
            code,
            status: "success".to_string(),
            data,
            message: message.into(),
        }
    }

    pub fn error(code: u16, data: T, message: impl Into<String>) -> Self {
        Self {
            code,
            status: "error".to_string(),
            data,
            message: message.into(),
        }
    }
}

