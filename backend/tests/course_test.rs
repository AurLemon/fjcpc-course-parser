// tests/course_test.rs
// 课程服务测试
use backend::parser::auth::get_user_info;
use backend::parser::schedule::{get_school_year, get_semester};
use backend::services::course::get_all_courses;
use backend::utils::config::AppConfig;
use backend::utils::http::create_http_client;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_course_service() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let test_ucode = config.test_student_ucode.clone()
        .expect("TEST_STUDENT_UCODE must be set in .env");

    println!("测试课程服务，使用 ucode: {}", test_ucode);

    // 创建 HTTP 客户端
    let client = create_http_client().await.expect("创建 HTTP 客户端失败");

    // 首先获取用户信息
    let user_info = match get_user_info(&test_ucode, &client, &config).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("获取用户信息失败: {}", e);
            return;
        }
    };

    // 获取学年信息
    let school_years = match get_school_year(&user_info.access_token, &client, &config).await {
        Ok(years) => years,
        Err(e) => {
            eprintln!("获取学年信息失败: {}", e);
            return;
        }
    };

    // 查找当前学期
    let current_semester = match school_years.iter().find(|s| s.is_current_semester) {
        Some(semester) => semester,
        None => {
            println!("未找到当前学期");
            return;
        }
    };

    // 获取学期周信息
    let semester_num_str = current_semester.semester.to_string();
    let semester = match get_semester(
        &user_info.access_token,
        &current_semester.school_year,
        &semester_num_str,
        &client,
        &config,
    ).await {
        Ok(sem) => sem,
        Err(e) => {
            eprintln!("获取学期信息失败: {}", e);
            return;
        }
    };

    // 获取所有课程
    match get_all_courses(
        &user_info.access_token,
        &user_info.student_id,
        &semester,
        &client,
        &config,
    ).await {
        Ok(all_courses) => {
            // 按周数排序，保证输出一致性
            let mut sorted_courses: Vec<_> = all_courses.into_iter().collect();
            sorted_courses.sort_by_key(|(week, _)| *week);

            let courses_json = serde_json::to_string_pretty(&sorted_courses.into_iter().collect::<std::collections::HashMap<_, _>>())
                .unwrap();

            println!("所有课程: {}", courses_json);

            // 保存到文件
            let file_path = Path::new("tests/data/all_courses.json");
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::write(file_path, courses_json).expect("写入文件失败");
            println!("已保存到 {:?}", file_path);
        }
        Err(e) => {
            eprintln!("获取所有课程失败: {}", e);
        }
    }
}

