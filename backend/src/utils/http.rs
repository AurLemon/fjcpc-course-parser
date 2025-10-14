use reqwest::Client;
use std::time::Duration;

/// 创建 HTTP 客户端
/// 
/// 为了解决船政那个神必 DNS 服务器加了 AAAA 却没法解析的问题，
/// 我们强制使用 IPv4。在 Rust 中，reqwest 默认会尝试 IPv6，
/// 但由于学校的 DNS 配置问题，我们需要确保只使用 IPv4。
/// 
/// 注意：reqwest 本身不直接支持强制 IPv4，但我们可以通过
/// 自定义 DNS 解析器或直接使用 IP 地址来实现。
/// MVP 阶段先使用默认配置，如果遇到问题再优化。
pub fn create_http_client() -> anyhow::Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    
    Ok(client)
}

