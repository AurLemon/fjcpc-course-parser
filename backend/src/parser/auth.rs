use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use tracing::error;

use crate::utils::config::AppConfig;
use crate::utils::simulator;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    /// 访问令牌（用于调用学校 API）
    pub access_token: String,
    /// 刷新令牌（用于更新访问令牌）
    pub refresh_token: String,
    /// 学号
    #[schema(example = "245810101")]
    pub student_id: String,
    /// 手机号
    #[schema(example = "138****1234")]
    pub student_phone: String,
    /// 真实姓名
    #[schema(example = "张三")]
    pub student_realname: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    user_info: UserInfoData,
}

#[derive(Debug, Deserialize)]
struct UserInfoData {
    username: String,
    phone: String,
    #[serde(rename = "nickName")]
    nick_name: String,
}

/// 获取 Basic 验证字符串（根据校内服务器数据推测，存在不确定性）
pub fn get_basic_auth() -> String {
    let username = "cat";
    let password = "cat";
    let credentials = format!("{}:{}", username, password);
    let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
    format!("Basic {}", encoded)
}

/// 获取 Basic 验证字符串（模拟浏览器环境，直接模拟学生访问课表以获取现实数据，非必要不用）
pub async fn get_server_basic_auth(raw_ucode: Option<String>, config: &AppConfig) -> Result<String> {
    let result = simulator::start_simulator(raw_ucode, config).await?;
    result
        .basic_auth_value
        .ok_or_else(|| anyhow::anyhow!("Failed to get basic auth from simulator"))
}

/// 获取 Bearer 验证字符串（优先通过浏览器模拟器捕获；失败则回退为调用 token 接口获取）
pub async fn get_server_bearer_auth(
    raw_ucode: &str,
    client: &Client,
    config: &AppConfig,
) -> Result<String> {
    // 先尝试浏览器模拟器
    if let Ok(sim_result) = simulator::start_simulator(Some(raw_ucode.to_string()), config).await {
        if let Some(bearer) = sim_result.bearer_auth_value {
            return Ok(bearer);
        }
    }
    // 回退：调用 token 接口，使用 access_token 构造 Bearer
    let user = get_user_info(raw_ucode, client, config).await?;
    Ok(format!("Bearer {}", user.access_token))
}


/// 传入 UCode 以获得用户信息
pub async fn get_user_info(
    raw_ucode: &str,
    client: &Client,
    config: &AppConfig,
) -> Result<UserInfo> {
    let request_url = format!("{}/gateway/auth/oauth/token", config.college_app_base_url);
    let ucode = format!("HUA_TENG-{}", raw_ucode);

    // 首次尝试使用静态 Basic Auth
    match try_get_user_info(&request_url, &ucode, &get_basic_auth(), client).await {
        Ok(user_info) => Ok(user_info),
        Err(e) => {
            // 如果是 401 错误，尝试使用浏览器模拟获取 Basic Auth
            if e.to_string().contains("401") {
                error!("Error while fetching info: {}", e);
                error!("Attempting retry with browser simulation...");

                let server_basic_auth = get_server_basic_auth(Some(raw_ucode.to_string()), config).await?;
                try_get_user_info(&request_url, &ucode, &server_basic_auth, client).await
            } else {
                Err(e)
            }
        }
    }
}

async fn try_get_user_info(
    request_url: &str,
    ucode: &str,
    auth: &str,
    client: &Client,
) -> Result<UserInfo> {
    let response = client
        .get(request_url)
        .header("Authorization", auth)
        .query(&[
            ("ucode", ucode),
            ("state", "1"),
            ("grant_type", "ucode"),
            ("scope", "server"),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Error while fetching info: {}. Message: {}",
            status,
            error_text
        ));
    }

    let token_response: TokenResponse = response.json().await?;

    Ok(UserInfo {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        student_id: token_response.user_info.username,
        student_phone: token_response.user_info.phone,
        student_realname: token_response.user_info.nick_name,
    })
}

