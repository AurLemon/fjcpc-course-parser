use serde::{Deserialize, Serialize};

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

