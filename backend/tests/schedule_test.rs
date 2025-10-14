// tests/schedule_test.rs
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

    println!("Testing schedule service with ucode: {}", test_ucode);

    // HTTP client
    let client = create_http_client().await.expect("Failed to create HTTP client");

    // Get user info first
    let user_info = match get_user_info(&test_ucode, &client, &config).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to get user info: {}", e);
            return;
        }
    };

    // Test get school year
    match get_school_year(&user_info.access_token, &client, &config).await {
        Ok(school_years) => {
            println!("School years: {:?}", school_years);

            // Find current semester
            if let Some(current_semester) = school_years.iter().find(|s| s.is_current_semester) {
                println!("Current semester: {:?}", current_semester);

                // Test get semester
                let semester_str = current_semester.semester.to_string();
                match get_semester(
                    &user_info.access_token,
                    &current_semester.school_year,
                    &semester_str,
                    &client,
                    &config,
                ).await {
                    Ok(semester) => {
                        println!("Semester weeks: {:?}", semester);

                        // Test get week course
                        match get_week_course(
                            &user_info.access_token,
                            &user_info.student_id,
                            &current_semester.start_time,
                            &client,
                            &config,
                        ).await {
                            Ok(week_course) => {
                                println!("Week course: {}", serde_json::to_string_pretty(&week_course).unwrap());
                            }
                            Err(e) => {
                                eprintln!("Failed to get week course: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get semester: {}", e);
                    }
                }
            } else {
                println!("No current semester found");
            }
        }
        Err(e) => {
            eprintln!("Failed to get school year: {}", e);
        }
    }
}

