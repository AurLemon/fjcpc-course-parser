use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::config::AppConfig;
use crate::utils::response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
struct CourseData {
    ucode: String,
}

/// GET /api/course
/// 当前仅返回测试用的 ucode（与 Node 后端保持一致）
pub async fn get_course(config: web::Data<AppConfig>) -> impl Responder {
    let ucode = config
        .test_student_ucode
        .clone()
        .unwrap_or_else(|| "NOT_SET".to_string());

    let data = CourseData { ucode };

    let response = ApiResponse::success(200, data, "Success");

    HttpResponse::Ok().json(response)
}

