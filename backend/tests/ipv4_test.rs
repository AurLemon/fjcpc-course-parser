// tests/ipv4_test.rs
use backend::utils::http::create_http_client;
use std::time::Instant;

#[tokio::test]
async fn test_force_ipv4() {
    println!("\n=== 测试强制 IPv4 连接（学校服务器）===");
    println!("学校服务器有 AAAA 记录但 IPv6 无法连接，会导致 10+ 秒超时");
    println!("如果强制 IPv4 成功，应该在 1-2 秒内完成连接\n");

    let client = create_http_client().await.expect("Failed to create client");

    // 直接测试学校服务器
    let school_url = "https://app.fjcpc.edu.cn";

    println!("测试 URL: {}", school_url);
    println!("开始计时...");

    let start = Instant::now();

    match client.get(school_url).send().await {
        Ok(response) => {
            let elapsed = start.elapsed();
            println!("\n✅ 连接成功！");
            println!("   状态码: {}", response.status());
            println!("   耗时: {:?}", elapsed);

            // 如果耗时超过 5 秒，说明可能尝试了 IPv6
            if elapsed.as_secs() > 5 {
                panic!("❌ 连接耗时过长 ({:?})，可能尝试了 IPv6！强制 IPv4 失败！", elapsed);
            } else {
                println!("\n🎉 测试通过！连接速度正常，确认强制使用了 IPv4！");
            }
        }
        Err(e) => {
            let elapsed = start.elapsed();
            println!("\n❌ 连接失败");
            println!("   耗时: {:?}", elapsed);
            println!("   错误: {}", e);

            if elapsed.as_secs() > 5 {
                panic!("连接失败且耗时过长，可能尝试了 IPv6 导致超时");
            } else {
                panic!("连接失败: {}", e);
            }
        }
    }
}



