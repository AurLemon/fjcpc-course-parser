// tests/auth_test.rs
use backend::parser::auth::{get_basic_auth, get_server_basic_auth, get_user_info};
use backend::utils::config::AppConfig;
use backend::utils::http::create_http_client;

#[tokio::test]
async fn test_auth_service() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let test_ucode = config.test_student_ucode.clone()
        .expect("TEST_STUDENT_UCODE must be set in .env");

    println!("Testing auth service with ucode: {}", test_ucode);

    // Create HTTP client
    let client = create_http_client().expect("Failed to create HTTP client");

    // Test basic auth
    let basic_auth = get_basic_auth();
    println!("Basic auth: {}", basic_auth);

    // Test server basic auth (browser simulation)
    match get_server_basic_auth(Some(test_ucode.clone()), &config).await {
        Ok(server_basic_auth) => {
            println!("Server basic auth: {}", server_basic_auth);
        }
        Err(e) => {
            eprintln!("Failed to get server basic auth: {}", e);
        }
    }

    // Test get user info
    match get_user_info(&test_ucode, &client, &config).await {
        Ok(user_info) => {
            println!("User info: {:?}", user_info);
        }
        Err(e) => {
            eprintln!("Failed to get user info: {}", e);
        }
    }

    // Extra: verify simulator bearer token (fallback to token API)
    match backend::parser::auth::get_server_bearer_auth(&test_ucode, &client, &config).await {
        Ok(bearer) => {
            println!("Simulator bearer: {}", bearer);
        }
        Err(e) => eprintln!("Failed to get simulator bearer: {}", e),
    }

}

