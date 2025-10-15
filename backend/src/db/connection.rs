use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;
use tracing::info;

pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./sqlite.db".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    let db = Database::connect(&database_url).await?;
    
    // 创建表（如果不存在）
    create_tables(&db).await?;
    
    info!("Database initialized successfully");
    Ok(db)
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::{ConnectionTrait, Statement};
    use sea_orm::DatabaseBackend;
    
    // 创建统计表
    let create_stats_table = r#"
        CREATE TABLE IF NOT EXISTS access_stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            total_requests INTEGER NOT NULL DEFAULT 0,
            unique_users INTEGER NOT NULL DEFAULT 0,
            last_updated_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        )
    "#;
    
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        create_stats_table.to_string(),
    ))
    .await?;
    
    // 创建日志表
    let create_logs_table = r#"
        CREATE TABLE IF NOT EXISTS request_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            duration_ms INTEGER NOT NULL,
            token TEXT NOT NULL,
            encrypted_student_id TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )
    "#;
    
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        create_logs_table.to_string(),
    ))
    .await?;
    
    // 创建用户访问记录表（用于统计唯一用户）
    let create_user_visits_table = r#"
        CREATE TABLE IF NOT EXISTS user_visits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ucode_hash TEXT NOT NULL UNIQUE,
            first_visit_at INTEGER NOT NULL,
            last_visit_at INTEGER NOT NULL,
            visit_count INTEGER NOT NULL DEFAULT 1
        )
    "#;
    
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        create_user_visits_table.to_string(),
    ))
    .await?;
    
    // 初始化统计表（如果为空）
    let init_stats = r#"
        INSERT OR IGNORE INTO access_stats (id, total_requests, unique_users, last_updated_at, created_at)
        VALUES (1, 0, 0, 0, 0)
    "#;
    
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        init_stats.to_string(),
    ))
    .await?;
    
    Ok(())
}

