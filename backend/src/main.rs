mod controller;
mod parser;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
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

    // 启动服务器
    let bind_address = "127.0.0.1:8080";
    info!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(routes::course::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
