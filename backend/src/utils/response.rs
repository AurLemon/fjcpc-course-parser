use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// HTTP 状态码
    pub code: u16,
    /// 响应状态（success 或 error）
    pub status: String,
    /// 响应数据
    pub data: T,
    /// 响应消息
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

