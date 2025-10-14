use utoipa::OpenApi;

use crate::controller;
use crate::parser::schedule::{CourseInfo, CourseSlot, DayCourse, SchoolYear, WeekInfo};
use crate::parser::auth::UserInfo;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FJCPC Course Parser Backend",
        version = "1.0.0",
        description = "福建船政交通职业学院课表解析后端 API\n\n**使用说明：**\n1. 所有接口都需要提供学生的 UCode\n2. UCode 可以从学校移动端应用中获取\n3. 课表数据来源于学校官方 API",
    ),
    paths(
        controller::schedule::post_schedule,
        controller::schedule::get_user_info_endpoint,
        controller::schedule::get_schedule_meta,
        controller::schedule::ping,
    ),
    components(schemas(
        controller::schedule::ScheduleRequest,
        controller::schedule::ScheduleResponse,
        controller::schedule::ScheduleMeta,
        SchoolYear,
        WeekInfo,
        DayCourse,
        CourseSlot,
        CourseInfo,
        UserInfo,
    )),
    tags(
        (name = "Schedule", description = "课表相关接口 - 提供课表查询、学年学期信息等功能"),
        (name = "Auth", description = "认证相关接口 - 提供用户信息查询功能"),
        (name = "Test", description = "测试接口 - 用于开发和调试")
    )
)]
pub struct ApiDoc;


