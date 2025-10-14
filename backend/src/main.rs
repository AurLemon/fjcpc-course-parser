mod controller;
mod parser;
mod routes;
mod services;
mod utils;
mod docs;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing::info;

use utils::config::AppConfig;
use utils::log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    // 初始化日志
    log::init_logger();

    // 加载配置
    let config = AppConfig::from_env();
    info!("Loaded config: {:?}", config);
    info!("Running in {:?} mode", config.app_env);

    // 启动服务器
    let bind_address = format!("127.0.0.1:{}", config.port);
    info!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let is_dev = config.is_development();

        let mut app = App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("/", web::get().to(move || async move {
                let env = if is_dev { "development" } else { "production" };
                let body = format!(
                    r#"{{"status":"ok","message":"FJCPC Course Parser API","version":"1.0.0","environment":"{}","endpoints":{{"api":"/api","docs":"/docs","openapi":"/api-doc/openapi.json"}}}}"#,
                    env
                );
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)
            }))
            .service(
                web::scope("/api")
                    .configure(routes::course::configure)
                    .configure(routes::schedule::configure)
            );

        // 仅在开发环境启用 Swagger UI
        if is_dev {
            app = app.service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-doc/openapi.json", crate::docs::ApiDoc::openapi())
            );
        }

        app
    })
    .bind(&bind_address)?
    .run()
    .await
}
