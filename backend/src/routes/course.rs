use actix_web::web;

use crate::controller::course;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/course", web::get().to(course::get_course));
}

