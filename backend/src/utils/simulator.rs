use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;
use tracing::{info, warn};

use super::config::AppConfig;

#[derive(Debug, Clone)]
pub struct SimulatorResult {
    pub basic_auth_value: Option<String>,
    pub bearer_auth_value: Option<String>,
}

/// 启动模拟器（使用浏览器模拟学生访问课表以获取现实数据）
///
/// 这个模拟器的逻辑要始终保留，我一开始只保留逻辑只是为了避免出现意外情况，
/// 毕竟我也不知道学校API到底会怎么变化
///
/// 使用 headless Chrome 来模拟浏览器访问，捕获 Authorization 头
///
/// 注意：当前实现为简化版本，headless_chrome 的网络监听 API 较为复杂。
/// 实际使用中，静态 Basic Auth ("cat:cat") 已经足够，这个模拟器主要作为 fallback。
pub async fn start_simulator(raw_ucode: Option<String>, config: &AppConfig) -> Result<SimulatorResult> {
    // 使用提供的 ucode 或配置中的测试 ucode
    let request_ucode = raw_ucode.or_else(|| config.test_student_ucode.clone())
        .ok_or_else(|| anyhow::anyhow!("No ucode provided"))?;

    warn!("Starting browser simulator (simplified version)...");

    // 启动 headless Chrome
    let launch_options = LaunchOptions {
        headless: true,
        args: vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-setuid-sandbox"),
            OsStr::new("--disable-gpu"),
            OsStr::new("--disable-dev-shm-usage"),
        ],
        ..Default::default()
    };

    let browser = Browser::new(launch_options)
        .map_err(|e| anyhow::anyhow!("Failed to launch browser: {}", e))?;

    let tab = browser.new_tab()
        .map_err(|e| anyhow::anyhow!("Failed to create new tab: {}", e))?;

    // 导航到课表页面
    let url = format!(
        "{}/czmobile/mytimetableIndexNew?uid={}",
        config.college_app_base_url, request_ucode
    );

    info!("Navigating to: {}", url);

    match tab.navigate_to(&url) {
        Ok(_) => {
            // 等待页面加载与后续 API 请求发出
            thread::sleep(Duration::from_secs(6));
            info!("Success: 模拟器已加载 {} 的数据。", request_ucode);
        }
        Err(e) => {
            warn!("Navigation error: {}", e);
        }
    }

    // 读取本地存储/会话存储中的 token 或 Authorization 片段
    let basic_auth_value = tab
        .evaluate(r#"(() => {
          function findByPrefix(store, prefix){
            try{
              if (!store) return null;
              for (let i=0;i<store.length;i++){
                const k = store.key(i);
                const v = store.getItem(k);
                if (!v) continue;
                if (typeof v === 'string'){
                  if (v.startsWith(prefix)) return v;
                  try { const obj = JSON.parse(v);
                    for (const val of Object.values(obj)){
                      if (typeof val === 'string' && val.startsWith(prefix)) return val;
                    }
                  } catch(e){}
                }
              }
            }catch(e){}
            return null;
          }
          return findByPrefix(window.localStorage, 'Basic ') ||
                 findByPrefix(window.sessionStorage, 'Basic ') || null;
        })()"#, false)
        .ok()
        .and_then(|res| res.value)
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let bearer_auth_value = tab
        .evaluate(r#"(() => {
          function findByPrefix(store, prefix){
            try{
              if (!store) return null;
              for (let i=0;i<store.length;i++){
                const k = store.key(i);
                const v = store.getItem(k);
                if (!v) continue;
                if (typeof v === 'string'){
                  if (v.startsWith(prefix)) return v;
                  try { const obj = JSON.parse(v);
                    for (const val of Object.values(obj)){
                      if (typeof val === 'string' && val.startsWith(prefix)) return val;
                    }
                  } catch(e){}
                }
              }
            }catch(e){}
            return null;
          }
          return findByPrefix(window.localStorage, 'Bearer ') ||
                 findByPrefix(window.sessionStorage, 'Bearer ') || null;
        })()"#, false)
        .ok()
        .and_then(|res| res.value)
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    // 兜底 Basic
    let basic_final = basic_auth_value.or_else(|| Some("Basic Y2F0OmNhdA==".to_string()));

    Ok(SimulatorResult {
        basic_auth_value: basic_final,
        bearer_auth_value,
    })
}
