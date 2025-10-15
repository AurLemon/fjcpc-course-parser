use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::schedule::DayCourse;

/// 缓存条目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    pub data: HashMap<u32, Vec<DayCourse>>,
    pub cached_at: u64, // Unix timestamp in seconds
}

/// 全局课表缓存（ucode -> 课表数据）
/// 缓存有效期：24 小时
static SCHEDULE_CACHE: Lazy<DashMap<String, CacheEntry>> = Lazy::new(|| DashMap::new());

const CACHE_TTL_SECONDS: u64 = 24 * 60 * 60; // 24 hours

/// 获取当前时间戳（秒）
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 检查缓存是否有效
fn is_cache_valid(cached_at: u64) -> bool {
    let now = current_timestamp();
    now - cached_at < CACHE_TTL_SECONDS
}

/// 从缓存获取课表数据
pub fn get_cached_schedule(ucode: &str) -> Option<HashMap<u32, Vec<DayCourse>>> {
    if let Some(entry) = SCHEDULE_CACHE.get(ucode) {
        if is_cache_valid(entry.cached_at) {
            return Some(entry.data.clone());
        } else {
            // 缓存过期，删除
            drop(entry);
            SCHEDULE_CACHE.remove(ucode);
        }
    }
    None
}

/// 设置课表缓存
pub fn set_cached_schedule(ucode: &str, data: HashMap<u32, Vec<DayCourse>>) {
    let entry = CacheEntry {
        data,
        cached_at: current_timestamp(),
    };
    SCHEDULE_CACHE.insert(ucode.to_string(), entry);
}

/// 清除指定用户的缓存
pub fn clear_cache(ucode: &str) {
    SCHEDULE_CACHE.remove(ucode);
}

/// 获取缓存统计信息
pub fn get_cache_stats() -> (usize, usize) {
    let total = SCHEDULE_CACHE.len();
    let valid = SCHEDULE_CACHE
        .iter()
        .filter(|entry| is_cache_valid(entry.cached_at))
        .count();
    (total, valid)
}

