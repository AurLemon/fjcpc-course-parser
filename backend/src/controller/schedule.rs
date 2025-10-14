use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::parser::{
    auth,
    schedule::{DayCourse, SchoolYear, WeekInfo},
};
use crate::services::course as course_service;
use crate::utils::{config::AppConfig, http::create_http_client, response::ApiResponse};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleRequest {
    pub ucode: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScheduleResponse {
    pub weeks: HashMap<u32, Vec<DayCourse>>,
}

/// POST /api/schedule
/// 传入 ucode，返回该学生当前学期全量课表（按周聚合）
#[utoipa::path(
    post,
    path = "/api/schedule",
    tag = "schedule",
    request_body = ScheduleRequest,
    responses(
        (status = 200, description = "OK", body = ApiResponse<ScheduleResponse>)
    )
)]
pub async fn post_schedule(
    config: web::Data<AppConfig>,
    payload: web::Json<ScheduleRequest>,
) -> impl Responder {
    let client = match create_http_client() {
        Ok(c) => c,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Create client failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let ucode = payload.ucode.clone();

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

    // 4) 并发获取所有周课程
    let weeks_map: HashMap<u32, Vec<DayCourse>> = match course_service::get_all_courses(
        &user.access_token,
        &user.student_id,
        &semester_weeks,
        &client,
        &config,
    ).await {
        Ok(m) => m,
        Err(e) => {
            let resp: ApiResponse<serde_json::Value> = ApiResponse::error(500, serde_json::json!({}), format!("Get all courses failed: {}", e));
            return HttpResponse::InternalServerError().json(resp);
        }
    };

    let data = ScheduleResponse { weeks: weeks_map };
    let resp = ApiResponse::success(200, data, "OK");
    HttpResponse::Ok().json(resp)
}

/// GET /api/auth/userinfo?ucode=xxx
/// 方便测试：返回用户基本信息
#[utoipa::path(
    get,
    path = "/api/auth/userinfo",
    tag = "auth",
    params(("ucode" = String, Query, description = "学生 UCode")),
    responses((status = 200, description = "OK", body = ApiResponse<auth::UserInfo>))
)]
pub async fn get_user_info_endpoint(
    config: web::Data<AppConfig>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let Some(ucode) = query.get("ucode").cloned() else {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Missing ucode");
        return HttpResponse::BadRequest().json(resp);
    };

    let client = match create_http_client() {
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
    pub years: Vec<SchoolYear>,
    pub weeks: Vec<WeekInfo>,
}

/// GET /api/schedule/meta?ucode=xxx
/// 方便测试：返回当前学年列表+当前学期周信息
#[utoipa::path(
    get,
    path = "/api/schedule/meta",
    tag = "schedule",
    params(("ucode" = String, Query, description = "学生 UCode")),
    responses((status = 200, description = "OK", body = ApiResponse<ScheduleMeta>))
)]
pub async fn get_schedule_meta(
    config: web::Data<AppConfig>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let Some(ucode) = query.get("ucode").cloned() else {
        let resp: ApiResponse<serde_json::Value> = ApiResponse::error(400, serde_json::json!({}), "Missing ucode");
        return HttpResponse::BadRequest().json(resp);
    };

    let client = match create_http_client() {
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

/// GET /api/ping
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success(200, serde_json::json!({"pong": true}), "OK"))
}

