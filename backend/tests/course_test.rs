// tests/course_test.rs
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

    println!("Testing course service with ucode: {}", test_ucode);

    // HTTP client
    let client = create_http_client().expect("Failed to create HTTP client");

    // Get user info first
    let user_info = match get_user_info(&test_ucode, &client, &config).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to get user info: {}", e);
            return;
        }
    };

    // Get school year
    let school_years = match get_school_year(&user_info.access_token, &client, &config).await {
        Ok(years) => years,
        Err(e) => {
            eprintln!("Failed to get school year: {}", e);
            return;
        }
    };

    // Find current semester
    let current_semester = match school_years.iter().find(|s| s.is_current_semester) {
        Some(semester) => semester,
        None => {
            println!("No current semester found");
            return;
        }
    };

    // Get semester weeks
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
            eprintln!("Failed to get semester: {}", e);
            return;
        }
    };

    // Get all courses
    match get_all_courses(
        &user_info.access_token,
        &user_info.student_id,
        &semester,
        &client,
        &config,
    ).await {
        Ok(all_courses) => {
            // Sort by week number for consistent output
            let mut sorted_courses: Vec<_> = all_courses.into_iter().collect();
            sorted_courses.sort_by_key(|(week, _)| *week);
            
            let courses_json = serde_json::to_string_pretty(&sorted_courses.into_iter().collect::<std::collections::HashMap<_, _>>())
                .unwrap();
            
            println!("All courses: {}", courses_json);

            // Save to file
            let file_path = Path::new("tests/data/all_courses.json");
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::write(file_path, courses_json).expect("Failed to write file");
            println!("Saved to {:?}", file_path);
        }
        Err(e) => {
            eprintln!("Failed to get all courses: {}", e);
        }
    }
}

