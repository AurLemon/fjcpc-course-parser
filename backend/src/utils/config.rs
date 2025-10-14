use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub college_app_base_url: String,
    pub test_student_ucode: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            college_app_base_url: env::var("FJCPC_APP_BASE_URL")
                .unwrap_or_else(|_| "https://app.fjcpc.edu.cn".to_string()),
            test_student_ucode: env::var("TEST_STUDENT_UCODE").ok(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

