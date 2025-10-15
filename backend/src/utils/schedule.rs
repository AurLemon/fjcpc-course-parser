use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike, Utc, FixedOffset};

/// 日程表对象
///
/// 根据学校作息时间表删减后手动转储为对象文件，
/// 考虑到项目只服务于我们船政所以就写死成导出模块到项目里方便直接用了
///
/// @author AurLemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub basic_info: BasicInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    /// 每天课程数
    pub total_lessons: u32,
    /// 课件时长，单位分钟
    pub lesson_duration: u32,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            basic_info: BasicInfo {
                total_lessons: 10,
                lesson_duration: 45,
            },
        }
    }
}

pub fn get_school_schedule() -> Schedule {
    Schedule::default()
}

/// 课程时间表（冬季/夏季）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTimeTable {
    pub times: Vec<(String, String)>, // (开始时间, 结束时间)
}

/// 判断日期是否为夏季作息时间
/// 夏季：6月1日 - 9月30日
/// 冬季：10月1日 - 次年5月31日
pub fn is_summer_schedule(date: &str) -> bool {
    if let Ok(parsed_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        let month = parsed_date.month();
        let day = parsed_date.day();

        // 6月1日 - 9月30日为夏季
        (month == 6 && day >= 1) || (month >= 7 && month <= 9)
    } else {
        // 解析失败，默认冬季
        false
    }
}

/// 获取课程时间表（根据日期自动判断冬夏季）
pub fn get_course_time_table(date: &str) -> CourseTimeTable {
    if is_summer_schedule(date) {
        // 夏季作息时间
        CourseTimeTable {
            times: vec![
                ("08:00".to_string(), "08:45".to_string()), // 第1节课
                ("08:55".to_string(), "09:40".to_string()), // 第2节课
                ("10:00".to_string(), "10:45".to_string()), // 第3节课
                ("10:55".to_string(), "11:40".to_string()), // 第4节课
                ("14:30".to_string(), "15:15".to_string()), // 第5节课
                ("15:25".to_string(), "16:10".to_string()), // 第6节课
                ("16:30".to_string(), "17:15".to_string()), // 第7节课
                ("17:25".to_string(), "18:10".to_string()), // 第8节课
                ("19:20".to_string(), "20:05".to_string()), // 第9节课
                ("20:15".to_string(), "21:00".to_string()), // 第10节课
            ],
        }
    } else {
        // 冬季作息时间
        CourseTimeTable {
            times: vec![
                ("08:00".to_string(), "08:45".to_string()), // 第1节课
                ("08:55".to_string(), "09:40".to_string()), // 第2节课
                ("10:00".to_string(), "10:45".to_string()), // 第3节课
                ("10:55".to_string(), "11:40".to_string()), // 第4节课
                ("14:00".to_string(), "14:45".to_string()), // 第5节课
                ("14:55".to_string(), "15:40".to_string()), // 第6节课
                ("16:00".to_string(), "16:45".to_string()), // 第7节课
                ("16:55".to_string(), "17:40".to_string()), // 第8节课
                ("19:00".to_string(), "19:45".to_string()), // 第9节课
                ("19:55".to_string(), "20:40".to_string()), // 第10节课
            ],
        }
    }
}



/// 获取冬季作息时间表
pub fn get_winter_course_time_table() -> CourseTimeTable {
    CourseTimeTable {
        times: vec![
            ("08:00".to_string(), "08:45".to_string()), // 第1节课
            ("08:55".to_string(), "09:40".to_string()), // 第2节课
            ("10:00".to_string(), "10:45".to_string()), // 第3节课
            ("10:55".to_string(), "11:40".to_string()), // 第4节课
            ("14:00".to_string(), "14:45".to_string()), // 第5节课
            ("14:55".to_string(), "15:40".to_string()), // 第6节课
            ("16:00".to_string(), "16:45".to_string()), // 第7节课
            ("16:55".to_string(), "17:40".to_string()), // 第8节课
            ("19:00".to_string(), "19:45".to_string()), // 第9节课
            ("19:55".to_string(), "20:40".to_string()), // 第10节课
        ],
    }
}

/// 获取夏季作息时间表
pub fn get_summer_course_time_table() -> CourseTimeTable {
    CourseTimeTable {
        times: vec![
            ("08:00".to_string(), "08:45".to_string()), // 第1节课
            ("08:55".to_string(), "09:40".to_string()), // 第2节课
            ("10:00".to_string(), "10:45".to_string()), // 第3节课
            ("10:55".to_string(), "11:40".to_string()), // 第4节课
            ("14:30".to_string(), "15:15".to_string()), // 第5节课
            ("15:25".to_string(), "16:10".to_string()), // 第6节课
            ("16:30".to_string(), "17:15".to_string()), // 第7节课
            ("17:25".to_string(), "18:10".to_string()), // 第8节课
            ("19:20".to_string(), "20:05".to_string()), // 第9节课
            ("20:15".to_string(), "21:00".to_string()), // 第10节课
        ],
    }
}

// --- Global East-8 timezone helpers (utils) ---
/// 固定东八区（Asia/Shanghai）
pub fn tz_east8() -> FixedOffset {
    FixedOffset::east_opt(8 * 3600).expect("valid east8 offset")
}

/// 东八区当前日期（YYYY-MM-DD）
pub fn east8_today_ymd() -> String {
    Utc::now().with_timezone(&tz_east8()).format("%Y-%m-%d").to_string()
}
