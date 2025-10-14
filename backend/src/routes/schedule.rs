use actix_web::web;

use crate::controller::schedule;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/schedule", web::post().to(schedule::post_schedule))
        .route("/auth/userinfo", web::get().to(schedule::get_user_info_endpoint))
        .route("/schedule/meta", web::get().to(schedule::get_schedule_meta))
        .route("/ping", web::get().to(schedule::ping));
}

