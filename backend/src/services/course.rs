use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

use crate::parser::course::get_all_courses as parser_get_all_courses;
use crate::parser::schedule::{DayCourse, WeekInfo};
use crate::utils::config::AppConfig;

/// 批量获取所有课程（业务层封装）
pub async fn get_all_courses(
    user_token: &str,
    student_id: &str,
    semester: &[WeekInfo],
    client: &Client,
    config: &AppConfig,
) -> Result<HashMap<u32, Vec<DayCourse>>> {
    parser_get_all_courses(user_token, student_id, semester, client, config).await
}

