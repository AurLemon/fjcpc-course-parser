use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use utoipa::ToSchema;

use crate::parser::{
    auth::{self, UserInfo},
    schedule::{DayCourse, SchoolYear, WeekInfo},
};
use crate::services::{course as course_service, stats};
use crate::utils::{
    cache, config::AppConfig, http::create_http_client, response::ApiResponse,
    schedule as schedule_utils,
};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleRequest {
    /// 学生 UCode
    ///
    /// 示例：`"ABC123DEF456GHI789JKL012MNO345PQR678"`
    #[schema(example = "ABC123DEF456GHI789JKL012MNO345PQR678")]
    pub ucode: String,
    /// 是否并行请求所有周（默认 true）
    #[serde(default)]
    pub parallel: Option<bool>,
    /// 是否使用缓存（默认 true；若命中直接返回缓存）
    #[serde(default)]
    pub use_cache: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleResponse {
    /// 课表数据，按周号聚合
    ///
    /// Key: 周号（1-22）
    /// Value: 该周的每日课程列表（周一到周日）
    pub weeks: HashMap<u32, Vec<DayCourse>>,
    /// 课程时间表
    pub time_table: Vec<(String, String)>,
    /// 明确的时令字段："winter" | "summer"
    pub season: String,
}

/// 课表 API 响应（具体类型，用于 OpenAPI 文档）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 课表数据
    pub data: ScheduleResponse,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 用户信息 API 响应（具体类型，用于 OpenAPI 文档）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfoApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 用户信息数据
    pub data: UserInfo,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 获取学生课表
///
/// 传入学生的 UCode，返回当前学期的完整课表数据（按周聚合）。
///
/// **功能说明：**
/// - 自动识别当前学期
/// - 并发获取所有周的课程数据
/// - 返回按周号聚合的课表
///
/// **返回数据：**
/// - 每周包含 7 天的课程（周一到周日）
/// - 每天包含多个课程时段
/// - 课程信息包括课程名称、教室、教师等详细信息
#[utoipa::path(
    post,
    path = "/api/schedule",
    tag = "Schedule",
    request_body = ScheduleRequest,
    responses(
        (status = 200, description = "成功获取课表数据", body = ScheduleApiResponse),
        (status = 400, description = "请求参数错误"),
        (status = 404, description = "未找到当前学期"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn post_schedule(
    config: web::Data<AppConfig>,
    db: web::Data<DatabaseConnection>,
    payload: web::Json<ScheduleRequest>,
) -> impl Responder {
    let start_time = Instant::now();
    let ucode = payload.ucode.clone();

    // 是否使用缓存（默认 true）
    let use_cache = payload.use_cache.unwrap_or(true);
    if use_cache {
        if let Some(cached_data) = cache::get_cached_schedule(&ucode) {
            tracing::info!("Cache hit for ucode: {}", ucode);
            // 东八区当前日期
            let today = schedule_utils::east8_today_ymd();
            let season = if schedule_utils::is_summer_schedule(&today) { "summer".to_string() } else { "winter".to_string() };
            let time_table = if season == "summer" { schedule_utils::get_summer_course_time_table().times } else { schedule_utils::get_winter_course_time_table().times };
            let data = ScheduleResponse { weeks: cached_data, time_table, season };
            let resp = ApiResponse::success(200, data, "OK (from cache)");
            return HttpResponse::Ok().json(resp);
        }
    }

    let client = match create_http_client().await {
        Ok(c) => c,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Create client failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    // 1) 获取用户信息
    let user = match auth::get_user_info(&ucode, &client, &config).await {
        Ok(u) => u,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get user info failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    // 2) 获取学年，定位当前学期
    let years = match crate::parser::schedule::get_school_year(&user.access_token, &client, &config).await {
        Ok(v) => v,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get school year failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let current = years.iter().rfind(|y| y.is_current_semester);
    let Some(current_semester) = current else {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(404, serde_json::json!({}), "No current semester found");
        return HttpResponse::NotFound().json(resp);
    };

    // 3) 获取学期周信息
    let semester_str = current_semester.semester.to_string();
    let semester_weeks: Vec<WeekInfo> = match crate::parser::schedule::get_semester(
        &user.access_token,
        &current_semester.school_year,
        &semester_str,
        &client,
        &config,
    ).await {
        Ok(v) => v,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get semester failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    // 4) 获取所有周课程（支持并行/顺序）
    let parallel = payload.parallel.unwrap_or(true);
    let mut weeks_map: HashMap<u32, Vec<DayCourse>> = match course_service::get_all_courses(
        &user.access_token,
        &user.student_id,
        &semester_weeks,
        &client,
        &config,
        parallel,
    ).await {
        Ok(m) => m,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get all courses failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    // 5) 对每周的课程按 weekday 排序
    for day_courses in weeks_map.values_mut() {
        day_courses.sort_by_key(|dc| dc.weekday);
    }

    // 设置缓存
    cache::set_cached_schedule(&ucode, weeks_map.clone());

    // 记录统计和日志
    let duration_ms = start_time.elapsed().as_millis() as i64;

    // 异步记录日志和统计（不阻塞响应）
    let db_clone = db.get_ref().clone();
    let ucode_clone = ucode.clone();
    let token_clone = user.access_token.clone();
    let student_id_clone = user.student_id.clone();

    tokio::spawn(async move {
        if let Err(e) = stats::log_request(&db_clone, &token_clone, &student_id_clone, duration_ms).await {
            tracing::error!("Failed to log request: {}", e);
        }
        if let Err(e) = stats::update_stats(&db_clone, &ucode_clone).await {
            tracing::error!("Failed to update stats: {}", e);
        }
    });

    // 确定时令与时间表（东八区当前日期）
    let today = schedule_utils::east8_today_ymd();
    let season = if schedule_utils::is_summer_schedule(&today) { "summer".to_string() } else { "winter".to_string() };
    let time_table = if season == "summer" { schedule_utils::get_summer_course_time_table().times } else { schedule_utils::get_winter_course_time_table().times };

    let data = ScheduleResponse {
        weeks: weeks_map,
        time_table,
        season,
    };
    let resp = ApiResponse::success(200, data, "OK");
    HttpResponse::Ok().json(resp)
}

/// 获取用户基本信息
///
/// 根据学生的 UCode 获取用户的基本信息，包括访问令牌、学号、姓名等。
///
/// **功能说明：**
/// - 用于测试和调试
/// - 返回用户的认证令牌和基本资料
///
/// **返回数据：**
/// - access_token: 访问令牌（用于后续 API 调用）
/// - refresh_token: 刷新令牌
/// - student_id: 学号
/// - student_phone: 手机号
/// - student_realname: 真实姓名
#[utoipa::path(
    get,
    path = "/api/auth/userinfo",
    tag = "Auth",
    params(
        ("ucode" = String, Query, description = "学生 UCode", example = "ABC123DEF456GHI789JKL012MNO345PQR678")
    ),
    responses(
        (status = 200, description = "成功获取用户信息", body = UserInfoApiResponse),
        (status = 400, description = "缺少 ucode 参数"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn get_user_info_endpoint(
    config: web::Data<AppConfig>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let Some(ucode) = query.get("ucode").cloned() else {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Missing ucode");
        return HttpResponse::BadRequest().json(resp);
    };

    let client = match create_http_client().await {
        Ok(c) => c,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Create client failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    match auth::get_user_info(&ucode, &client, &config).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(200, user, "OK")),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::error(500, serde_json::json!({}), format!("Get user info failed: {}", e))),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleMeta {
    /// 学年列表（包含所有历史学年和当前学年）
    pub years: Vec<SchoolYear>,
    /// 当前学期的周信息列表
    pub weeks: Vec<WeekInfo>,
}

/// 学年元数据 API 响应（具体类型，用于 OpenAPI 文档）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleMetaApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 学年和周信息数据
    pub data: ScheduleMeta,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// Ping 响应数据
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PingData {
    /// Pong 标识
    #[schema(example = true)]
    pub pong: bool,
}

/// Ping API 响应（具体类型，用于 OpenAPI 文档）
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PingApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// Ping 数据
    pub data: PingData,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 获取学年和学期元数据
///
/// 返回所有学年列表和当前学期的周信息，用于了解学期结构。
///
/// **功能说明：**
/// - 获取所有历史学年和当前学年
/// - 获取当前学期的所有周信息（包括起止日期）
/// - 用于测试和调试
///
/// **返回数据：**
/// - years: 学年列表（标记当前学期）
/// - weeks: 周信息列表（周号、起止日期）
#[utoipa::path(
    get,
    path = "/api/schedule/meta",
    tag = "Schedule",
    params(
        ("ucode" = String, Query, description = "学生 UCode", example = "ABC123DEF456GHI789JKL012MNO345PQR678")
    ),
    responses(
        (status = 200, description = "成功获取元数据", body = ScheduleMetaApiResponse),
        (status = 400, description = "缺少 ucode 参数"),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn get_schedule_meta(
    config: web::Data<AppConfig>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let Some(ucode) = query.get("ucode").cloned() else {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Missing ucode");
        return HttpResponse::BadRequest().json(resp);
    };

    let client = match create_http_client().await {
        Ok(c) => c,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Create client failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let user = match auth::get_user_info(&ucode, &client, &config).await {
        Ok(u) => u,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get user info failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let years = match crate::parser::schedule::get_school_year(&user.access_token, &client, &config).await {
        Ok(v) => v,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get school year failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let current = years.iter().rfind(|y| y.is_current_semester);
    let Some(current_semester) = current else {
        let meta = ScheduleMeta { years, weeks: vec![] };
        let resp = ApiResponse::success(200, meta, "OK (no current semester)");

        return HttpResponse::Ok().json(resp);
    };
    let semester_str = current_semester.semester.to_string();
    let weeks = match crate::parser::schedule::get_semester(
        &user.access_token,
        &current_semester.school_year,
        &semester_str,
        &client,
        &config,
    ).await {
        Ok(v) => v,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get semester failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let meta = ScheduleMeta { years, weeks };
    HttpResponse::Ok().json(ApiResponse::success(200, meta, "OK"))
}

/// 健康检查
///
/// 用于检查 API 服务是否正常运行。
///
/// **功能说明：**
/// - 简单的健康检查端点
/// - 返回 pong 表示服务正常
#[utoipa::path(
    get,
    path = "/api/ping",
    tag = "Test",
    responses(
        (status = 200, description = "服务正常运行", body = PingApiResponse)
    )
)]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(200, serde_json::json!({"pong": true}), "OK"))
}

/// 统计信息响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StatsApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 统计数据
    pub data: stats::StatsResponse,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 获取访问统计信息
///
/// 返回 API 的访问统计数据，包括总请求数、唯一用户数等。
///
/// **功能说明：**
/// - 统计所有访问 /api/schedule 接口的请求
/// - 保护用户隐私，仅统计数量
#[utoipa::path(
    get,
    path = "/api/stats",
    tag = "Stats",
    responses(
        (status = 200, description = "成功获取统计信息", body = StatsApiResponse),
        (status = 500, description = "服务器内部错误")
    )
)]
pub async fn get_stats(db: web::Data<DatabaseConnection>) -> impl Responder {
    match stats::get_stats(db.get_ref()).await {
        Ok(data) => {
            let resp = ApiResponse::success(200, data, "OK");
            HttpResponse::Ok().json(resp)
        }
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(
                500,
                serde_json::json!({}),

                format!("Failed to get stats: {}", e),
            );
            HttpResponse::InternalServerError().json(resp)
        }
    }
}



#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimeTableResponse {
    /// 课程时间表
    pub time_table: Vec<(String, String)>,
    /// 明确的时令字段："winter" | "summer"
    pub season: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SeasonResponse {
    /// 当前时令："winter" | "summer"
    pub season: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SeasonApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 时令数据
    pub data: SeasonResponse,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 获取作息时令（支持可选日期），默认按东八区当前日期
#[utoipa::path(
    get,
    path = "/api/season",
    tag = "Schedule",
    params(
        ("date" = String, Query, description = "计算该日期的时令，格式 YYYY-MM-DD；不传则按东八区当前日期", example = "2025-06-01")
    ),
    responses(
        (status = 200, description = "成功获取当前时令", body = SeasonApiResponse),
        (status = 400, description = "日期格式不合法，应为 YYYY-MM-DD")
    )
)]
pub async fn get_season(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let date_str = if let Some(d) = query.get("date") {
        d.to_string()
    } else {
        schedule_utils::east8_today_ymd()
    };

    if chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").is_err() {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Invalid date format, expected YYYY-MM-DD");
        return HttpResponse::BadRequest().json(resp);
    }

    let season = if schedule_utils::is_summer_schedule(&date_str) { "summer".to_string() } else { "winter".to_string() };
    HttpResponse::Ok().json(ApiResponse::success(200, SeasonResponse { season }, "OK"))
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimeTableApiResponse {
    /// HTTP 状态码
    #[schema(example = 200)]
    pub code: u16,
    /// 响应状态
    #[schema(example = "success")]
    pub status: String,
    /// 时间表数据
    pub data: TimeTableResponse,
    /// 响应消息
    #[schema(example = "OK")]
    pub message: String,
}

/// 获取课程时间表（支持 auto | winter | summer）
#[utoipa::path(
    get,
    path = "/api/time-table",
    tag = "Schedule",
    params(("date" = String, Query, description = "YYYY-MM-DD；不传则按东八区当天", example = "2025-06-01")),
    responses(
        (status = 200, description = "成功获取时间表", body = TimeTableApiResponse),
        (status = 400, description = "请求参数错误")
    )
)]
pub async fn get_time_table(
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let date_str = if let Some(d) = query.get("date") {
        d.to_string()
    } else {
        schedule_utils::east8_today_ymd()
    };

    if chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").is_err() {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Invalid date format, expected YYYY-MM-DD");
        return HttpResponse::BadRequest().json(resp);
    }

    let season = if schedule_utils::is_summer_schedule(&date_str) { "summer".to_string() } else { "winter".to_string() };
    let time_table = if season == "summer" { schedule_utils::get_summer_course_time_table().times } else { schedule_utils::get_winter_course_time_table().times };

    HttpResponse::Ok().json(ApiResponse::success(200, TimeTableResponse { time_table, season }, "OK"))
}
