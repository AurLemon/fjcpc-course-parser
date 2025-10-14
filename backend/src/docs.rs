use utoipa::OpenApi;

use crate::controller;
use crate::parser::schedule::{CourseInfo, CourseSlot, DayCourse, SchoolYear, WeekInfo};
use crate::parser::auth::UserInfo;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::schedule::post_schedule,
        controller::schedule::get_user_info_endpoint,
        controller::schedule::get_schedule_meta,
        controller::course::get_course,
    ),
    components(schemas(
        controller::schedule::ScheduleRequest,
        controller::schedule::ScheduleResponse,
        SchoolYear,
        WeekInfo,
        DayCourse,
        CourseSlot,
        CourseInfo,
        UserInfo,
    )),
    tags(
        (name = "schedule", description = "Schedule related endpoints"),
        (name = "auth", description = "Auth related endpoints"),
        (name = "test", description = "Test endpoints")
    )
)]
pub struct ApiDoc;


