use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

use crate::parser::course::get_all_courses as parser_get_all_courses;
use crate::parser::schedule::{self, DayCourse, WeekInfo};
use crate::utils::config::AppConfig;

/// 批量获取所有课程（业务层封装）
/// parallel=true 并发；false 顺序
pub async fn get_all_courses(
    user_token: &str,
    student_id: &str,
    semester: &[WeekInfo],
    client: &Client,
    config: &AppConfig,
    parallel: bool,
) -> Result<HashMap<u32, Vec<DayCourse>>> {
    if parallel {
        return parser_get_all_courses(user_token, student_id, semester, client, config).await;
    }
    // 顺序请求（按周依次获取）
    let mut courses_map = HashMap::new();
    for week in semester {
        let data = schedule::get_week_course(
            user_token,
            student_id,
            &week.start_time,
            client,
            config,
        )
        .await?;
        courses_map.insert(week.week, data);
    }
    Ok(courses_map)
}

