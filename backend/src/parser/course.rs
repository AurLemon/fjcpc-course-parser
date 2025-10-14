use anyhow::Result;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use std::collections::HashMap;
use tracing::{error, info};

use super::schedule::{get_week_course, DayCourse, WeekInfo};
use crate::utils::config::AppConfig;

/// 批量获取所有课程
pub async fn get_all_courses(
    user_token: &str,
    student_id: &str,
    semester: &[WeekInfo],
    client: &Client,
    config: &AppConfig,
) -> Result<HashMap<u32, Vec<DayCourse>>> {
    if semester.is_empty() {
        return Err(anyhow::anyhow!("Semester info must not be empty"));
    }

    if student_id.is_empty() || user_token.is_empty() {
        return Err(anyhow::anyhow!("Student ID or User token must be provided"));
    }

    let mut courses_map = HashMap::new();
    let total_weeks = semester.len();

    // 创建并发任务
    let mut futures = FuturesUnordered::new();

    for week in semester {
        let user_token = user_token.to_string();
        let student_id = student_id.to_string();
        let week_clone = week.clone();
        let client_clone = client.clone();
        let config_clone = config.clone();

        futures.push(async move {
            info!(
                "Student {} ({}) are requesting week course. ({} / {})",
                student_id, user_token, week_clone.week, total_weeks
            );

            match get_week_course(
                &user_token,
                &student_id,
                &week_clone.start_time,
                &client_clone,
                &config_clone,
            )
            .await
            {
                Ok(week_course_data) => {
                    info!(
                        "Student {} ({}) requested week {} course successfully.",
                        student_id, user_token, week_clone.week
                    );
                    Some((week_clone.week, week_course_data))
                }
                Err(e) => {
                    error!(
                        "Student {} ({}) failed to request week course: {}",
                        student_id, user_token, e
                    );
                    None
                }
            }
        });
    }

    // 收集所有结果
    while let Some(result) = futures.next().await {
        if let Some((week, data)) = result {
            courses_map.insert(week, data);
        }
    }

    Ok(courses_map)
}

