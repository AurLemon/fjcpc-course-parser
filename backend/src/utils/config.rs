use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum AppEnv {
    Development,
    Production,
}

impl AppEnv {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => AppEnv::Production,
            _ => AppEnv::Development,
        }
    }

    pub fn is_development(&self) -> bool {
        *self == AppEnv::Development
    }

    pub fn is_production(&self) -> bool {
        *self == AppEnv::Production
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub port: u16,
    pub college_app_base_url: String,
    pub test_student_ucode: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let app_env = env::var("APP_ENV")
            .map(|s| AppEnv::from_str(&s))
            .unwrap_or(AppEnv::Development);

        let port = env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);

        Self {
            app_env,
            port,
            college_app_base_url: env::var("FJCPC_APP_BASE_URL")
                .unwrap_or_else(|_| "https://app.fjcpc.edu.cn".to_string()),
            test_student_ucode: env::var("TEST_STUDENT_UCODE").ok(),
        }
    }

    pub fn is_development(&self) -> bool {
        self.app_env.is_development()
    }

    pub fn is_production(&self) -> bool {
        self.app_env.is_production()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

