use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use reqwest::Client;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::info;

/// DNS 缓存条目
#[derive(Clone, Debug)]
struct DnsCacheEntry {
    ip: String,
    cached_at: SystemTime,
}

/// 全局 DNS 缓存（域名 -> IPv4 地址）
/// 缓存有效期：7 天
static DNS_CACHE: once_cell::sync::Lazy<Arc<Mutex<std::collections::HashMap<String, DnsCacheEntry>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

const DNS_CACHE_TTL: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 7 天

/// 解析域名的 IPv4 地址（只查询 A 记录，完全忽略 AAAA）
///
/// 使用 hickory-resolver 进行 DNS 查询，只请求 A 记录（IPv4），
/// 不会触发 AAAA 记录（IPv6）查询，避免 IPv6 超时。
/// 结果会缓存 7 天，避免频繁 DNS 查询。
async fn resolve_ipv4(domain: &str) -> anyhow::Result<String> {
    // 1. 检查缓存
    {
        let cache = DNS_CACHE.lock().unwrap();
        if let Some(entry) = cache.get(domain) {
            if let Ok(elapsed) = entry.cached_at.elapsed() {
                if elapsed < DNS_CACHE_TTL {
                    info!("DNS cache hit for {}: {} (age: {:?})", domain, entry.ip, elapsed);
                    return Ok(entry.ip.clone());
                } else {
                    info!("DNS cache expired for {} (age: {:?})", domain, elapsed);
                }
            }
        }
    }

    // 2. 执行 DNS 解析（只查询 A 记录，不查询 AAAA）
    info!("Resolving IPv4 address for {} (A record only)...", domain);

    // 使用系统默认 DNS 配置
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );

    // 只查询 A 记录（IPv4）
    let response = resolver.ipv4_lookup(domain).await?;

    // 获取第一个 IPv4 地址
    let ipv4 = response
        .iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No A record found for {}", domain))?
        .to_string();

    info!("Resolved {} to IPv4: {} (A record only, no AAAA query)", domain, ipv4);

    // 3. 更新缓存
    {
        let mut cache = DNS_CACHE.lock().unwrap();
        cache.insert(
            domain.to_string(),
            DnsCacheEntry {
                ip: ipv4.clone(),
                cached_at: SystemTime::now(),
            },
        );
    }

    Ok(ipv4)
}

/// 创建 HTTP 客户端（强制 IPv4）
///
/// 为了解决船政那个神必 DNS 服务器加了 AAAA 却没法解析的问题，
/// 我们强制使用 IPv4。通过以下方式确保只使用 IPv4：
/// 1. 绑定本地地址为 0.0.0.0（IPv4 UNSPECIFIED）
/// 2. 使用 hickory-resolver 只查询 A 记录（IPv4），完全不查询 AAAA（IPv6）
/// 3. 使用 resolve() 预解析，跳过 reqwest 的 DNS 查询
/// 4. DNS 结果缓存 7 天，避免频繁查询
///
/// 这样可以完全避免 IPv6 超时导致的 10+ 秒延迟。
pub async fn create_http_client() -> anyhow::Result<Client> {
    let domain = "app.fjcpc.edu.cn";

    // 动态解析 IPv4 地址（只查询 A 记录，带缓存）
    let ipv4 = resolve_ipv4(domain).await?;
    let socket_addr = format!("{}:443", ipv4).parse::<SocketAddr>()?;

    info!("Creating HTTP client with IPv4 binding: {} -> {}", domain, socket_addr);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .local_address(IpAddr::V4(Ipv4Addr::UNSPECIFIED))  // 0.0.0.0，强制 IPv4
        .resolve(domain, socket_addr)  // 强制域名解析到 IPv4 地址
        .build()?;

    Ok(client)
}

