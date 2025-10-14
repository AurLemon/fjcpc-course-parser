// tests/schedule_test.rs
// 课表服务测试
use backend::parser::auth::get_user_info;
use backend::parser::schedule::{get_school_year, get_semester, get_week_course};
use backend::utils::config::AppConfig;
use backend::utils::http::create_http_client;

#[tokio::test]
async fn test_schedule_service() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let test_ucode = config.test_student_ucode.clone()
        .expect("TEST_STUDENT_UCODE must be set in .env");

    println!("测试课表服务，使用 ucode: {}", test_ucode);

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

    // 测试获取学年信息
    match get_school_year(&user_info.access_token, &client, &config).await {
        Ok(school_years) => {
            println!("学年信息: {:?}", school_years);

            // 查找当前学期
            if let Some(current_semester) = school_years.iter().find(|s| s.is_current_semester) {
                println!("当前学期: {:?}", current_semester);

                // 测试获取学期周信息
                let semester_str = current_semester.semester.to_string();
                match get_semester(
                    &user_info.access_token,
                    &current_semester.school_year,
                    &semester_str,
                    &client,
                    &config,
                ).await {
                    Ok(semester) => {
                        println!("学期周信息: {:?}", semester);

                        // 测试获取周课程
                        match get_week_course(
                            &user_info.access_token,
                            &user_info.student_id,
                            &current_semester.start_time,
                            &client,
                            &config,
                        ).await {
                            Ok(week_course) => {
                                println!("周课程: {}", serde_json::to_string_pretty(&week_course).unwrap());
                            }
                            Err(e) => {
                                eprintln!("获取周课程失败: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("获取学期信息失败: {}", e);
                    }
                }
            } else {
                println!("未找到当前学期");
            }
        }
        Err(e) => {
            eprintln!("获取学年信息失败: {}", e);
        }
    }
}

