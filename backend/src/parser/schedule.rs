use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::utils::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolYear {
    pub school_year: String,
    pub semester: u32,
    pub is_current_semester: bool,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekInfo {
    pub week: u32,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayCourse {
    pub weekday: u32,
    pub course: Vec<CourseSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSlot {
    pub course_number: u32,
    pub course_info: Option<CourseInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseInfo {
    pub name: String,
    pub classroom: Option<String>,
    pub class: String,
    pub teacher: Vec<String>,
    pub course_number: u32,
    pub weekday: u32,
    pub color: String,
    pub continuous_course: u32,
    pub code: String,
}

#[derive(Debug, Deserialize)]
struct SchoolYearResponse {
    data: Vec<SchoolYearData>,
}

#[derive(Debug, Deserialize)]
struct SchoolYearData {
    xn: String,
    xq: String,
    dqxqbj: String,
    qsrq: String,
    jsrq: String,
}

#[derive(Debug, Deserialize)]
struct SemesterResponse {
    data: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct WeekCourseResponse {
    data: Vec<Vec<String>>,
}

/// 获取系统内有记录的学年数据（学期起始日和结束日、周数）
pub async fn get_school_year(user_token: &str, client: &Client, config: &AppConfig) -> Result<Vec<SchoolYear>> {
    let school_year_url = format!("{}/gateway/xgwork/appCourseTable/getXn", config.college_app_base_url);

    let response = client
        .get(&school_year_url)
        .header("Authorization", format!("Bearer {}", user_token))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        error!("Student {} failed to request school year. Error: {} - {}", user_token, status, error_text);
        return Err(anyhow::anyhow!("Error while fetching info: {}. Message: {}", status, error_text));
    }

    let school_year_response: SchoolYearResponse = response.json().await?;

    let formatted_data = school_year_response
        .data
        .into_iter()
        .map(|item| SchoolYear {
            school_year: item.xn,
            semester: item.xq.parse().unwrap_or(0),
            is_current_semester: item.dqxqbj.parse::<u32>().unwrap_or(0) != 0,
            start_time: item.qsrq,
            end_time: item.jsrq,
        })
        .collect();

    Ok(formatted_data)
}

/// 获取指定学期所有周的起始日
pub async fn get_semester(
    user_token: &str,
    school_year: &str,
    semester: &str,
    client: &Client,
    config: &AppConfig,
) -> Result<Vec<WeekInfo>> {
    let semester_url = format!("{}/gateway/xgwork/appCourseTable/getSemesterbyXn", config.college_app_base_url);

    let response = client
        .get(&semester_url)
        .header("Authorization", format!("Bearer {}", user_token))
        .query(&[("xn", school_year), ("xq", semester)])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        error!("Student {} failed to request semester info. Error: {} - {}", user_token, status, error_text);
        return Err(anyhow::anyhow!("Error while fetching info: {}. Message: {}", status, error_text));
    }

    let semester_response: SemesterResponse = response.json().await?;

    let formatted_data = semester_response
        .data
        .into_iter()
        .map(|item| WeekInfo {
            week: item.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            start_time: item.get(1).cloned().unwrap_or_default(),
            end_time: item.get(2).cloned().unwrap_or_default(),
        })
        .collect();

    Ok(formatted_data)
}

/// 获取周课程
pub async fn get_week_course(
    user_token: &str,
    student_id: &str,
    start_time: &str,
    client: &Client,
    config: &AppConfig,
) -> Result<Vec<DayCourse>> {
    let semester_url = format!("{}/gateway/xgwork/appCourseTable/getListByNoWeek2", config.college_app_base_url);

    let response = client
        .get(&semester_url)
        .header("Authorization", format!("Bearer {}", user_token))
        .query(&[("no", student_id), ("startDate", start_time)])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        error!("Student {} failed to request week course. Error: {} - {}", user_token, status, error_text);
        return Err(anyhow::anyhow!("Error while fetching info: {}. Message: {}", status, error_text));
    }

    let week_course_response: WeekCourseResponse = response.json().await?;

    let formatted_data = week_course_response
        .data
        .into_iter()
        .enumerate()
        .map(|(day_index, day_courses)| DayCourse {
            weekday: (day_index + 1) as u32,
            course: day_courses
                .into_iter()
                .enumerate()
                .map(|(course_index, course_str)| CourseSlot {
                    course_number: (course_index + 1) as u32,
                    course_info: parse_course_string(&course_str),
                })
                .collect(),
        })
        .collect();

    Ok(formatted_data)
}

/// 解析课程信息（把课程信息字符串解析成可读的对象）
fn parse_course_string(string: &str) -> Option<CourseInfo> {
    if string.is_empty() {
        return None;
    }

    let split_data: Vec<&str> = string.split('|').collect();
    if split_data.len() < 9 {
        return None;
    }

    Some(CourseInfo {
        name: split_data[0].to_string(),
        classroom: if split_data[1] == "无" {
            None
        } else {
            Some(split_data[1].to_string())
        },
        class: split_data[2].to_string(),
        teacher: split_data[3].split(';').map(|s| s.to_string()).collect(),
        course_number: split_data[4].parse().unwrap_or(0),
        weekday: split_data[5].parse().unwrap_or(0),
        color: split_data[6].to_string(),
        continuous_course: split_data[7].parse().unwrap_or(0),
        code: split_data[8].to_string(),
    })
}

