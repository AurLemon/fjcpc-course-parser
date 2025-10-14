// tests/auth_test.rs
// 认证服务测试
use backend::parser::auth::{get_basic_auth, get_server_basic_auth, get_user_info};
use backend::utils::config::AppConfig;
use backend::utils::http::create_http_client;

#[tokio::test]
async fn test_auth_service() {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let test_ucode = config.test_student_ucode.clone()
        .expect("TEST_STUDENT_UCODE must be set in .env");

    println!("测试认证服务，使用 ucode: {}", test_ucode);

    // 创建 HTTP 客户端
    let client = create_http_client().await.expect("创建 HTTP 客户端失败");

    // 测试静态 Basic Auth
    let basic_auth = get_basic_auth();
    println!("静态 Basic Auth: {}", basic_auth);

    // 测试服务器 Basic Auth（浏览器模拟）
    match get_server_basic_auth(Some(test_ucode.clone()), &config).await {
        Ok(server_basic_auth) => {
            println!("服务器 Basic Auth: {}", server_basic_auth);
        }
        Err(e) => {
            eprintln!("获取服务器 Basic Auth 失败: {}", e);
        }
    }

    // 测试获取用户信息
    match get_user_info(&test_ucode, &client, &config).await {
        Ok(user_info) => {
            println!("用户信息: {:?}", user_info);
        }
        Err(e) => {
            eprintln!("获取用户信息失败: {}", e);
        }
    }

    // 额外测试：验证模拟器 Bearer Token（回退到 Token API）
    match backend::parser::auth::get_server_bearer_auth(&test_ucode, &client, &config).await {
        Ok(bearer) => {
            println!("模拟器 Bearer Token: {}", bearer);
        }
        Err(e) => eprintln!("获取模拟器 Bearer Token 失败: {}", e),
    }

}

