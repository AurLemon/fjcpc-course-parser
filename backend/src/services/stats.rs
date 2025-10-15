use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::models::{access_stats, request_logs, user_visits};
use crate::utils::crypto::{encrypt_student_id, hash_ucode};

/// 获取当前时间戳（毫秒）
fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// 记录请求日志
pub async fn log_request(
    db: &DatabaseConnection,
    token: &str,
    student_id: &str,
    duration_ms: i64,
) -> Result<()> {
    let timestamp = current_timestamp_millis();
    
    // 使用 timestamp 加密学号
    let encrypted_student_id = encrypt_student_id(student_id, timestamp)?;
    
    let log = request_logs::ActiveModel {
        timestamp: Set(timestamp),
        duration_ms: Set(duration_ms),
        token: Set(token.to_string()),
        encrypted_student_id: Set(encrypted_student_id),
        created_at: Set(timestamp),
        ..Default::default()
    };
    
    log.insert(db).await?;
    Ok(())
}

/// 更新访问统计
pub async fn update_stats(db: &DatabaseConnection, ucode: &str) -> Result<()> {
    let timestamp = current_timestamp_millis();
    let ucode_hash = hash_ucode(ucode);
    
    // 检查用户是否首次访问
    let existing_user = user_visits::Entity::find()
        .filter(user_visits::Column::UcodeHash.eq(&ucode_hash))
        .one(db)
        .await?;
    
    let is_new_user = if let Some(user) = existing_user {
        // 更新现有用户
        let mut active_user: user_visits::ActiveModel = user.clone().into();
        active_user.last_visit_at = Set(timestamp);
        active_user.visit_count = Set(user.visit_count + 1);
        active_user.update(db).await?;
        false
    } else {
        // 新用户
        let new_user = user_visits::ActiveModel {
            ucode_hash: Set(ucode_hash),
            first_visit_at: Set(timestamp),
            last_visit_at: Set(timestamp),
            visit_count: Set(1),
            ..Default::default()
        };
        new_user.insert(db).await?;
        true
    };
    
    // 更新总体统计
    let stats = access_stats::Entity::find()
        .filter(access_stats::Column::Id.eq(1))
        .one(db)
        .await?;
    
    if let Some(stats) = stats {
        let mut active_stats: access_stats::ActiveModel = stats.into();
        active_stats.total_requests = Set(active_stats.total_requests.unwrap() + 1);
        if is_new_user {
            active_stats.unique_users = Set(active_stats.unique_users.unwrap() + 1);
        }
        active_stats.last_updated_at = Set(timestamp);
        active_stats.update(db).await?;
    }
    
    Ok(())
}

/// 获取统计信息
pub async fn get_stats(db: &DatabaseConnection) -> Result<StatsResponse> {
    let stats = access_stats::Entity::find()
        .filter(access_stats::Column::Id.eq(1))
        .one(db)
        .await?;
    
    if let Some(stats) = stats {
        Ok(StatsResponse {
            total_requests: stats.total_requests,
            unique_users: stats.unique_users,
            last_updated_at: stats.last_updated_at,
        })
    } else {
        Ok(StatsResponse {
            total_requests: 0,
            unique_users: 0,
            last_updated_at: 0,
        })
    }
}

/// 统计响应
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct StatsResponse {
    /// 总请求数
    #[schema(example = 1234)]
    pub total_requests: i32,
    /// 唯一用户数
    #[schema(example = 567)]
    pub unique_users: i32,
    /// 最后更新时间（毫秒时间戳）
    #[schema(example = "1704067200000")]
    pub last_updated_at: i64,
}

